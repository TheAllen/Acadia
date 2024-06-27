use std::{net::SocketAddr, time::Duration};

use axum::{extract::{Path, Query, State}, http::StatusCode, response::IntoResponse, routing::get, Json, Router};
use chrono::{DateTime, Utc};
use dotenv::dotenv;
use serde::{Deserialize, Serialize};
use sqlx::{postgres::PgPoolOptions, types::chrono::{NaiveDate, NaiveDateTime}, PgPool};
use bigdecimal::{ToPrimitive};

use bigdecimal::BigDecimal as BigDecimalExternal;
use sqlx::types::BigDecimal;


/* Entities */ 
#[derive(Debug, Clone, Deserialize, Serialize, sqlx::FromRow)]
struct Stock {
    pub stock_id: i32,
    pub symbol: String,
    pub name: String,
    pub exchange: String,
    pub sector: Option<String>,
    pub industry: Option<String>
}

#[derive(Debug, Clone, Deserialize, Serialize)]
struct CreateStock {
    pub symbol: String,
    pub name: String,
    pub exchange: String,
    pub sector: Option<String>,
    pub industry: Option<String>
}

impl From<Json<Stock>> for Stock {
    fn from(stock: Json<Stock>) -> Self {
        Stock {
            stock_id: stock.stock_id,
            symbol: stock.symbol.clone(),
            name: stock.name.clone(),
            exchange: stock.exchange.clone(),
            sector: stock.sector.clone(),
            industry: stock.industry.clone(),
        }
    }
}

impl From<axum::Json<CreateStock>> for CreateStock {
    fn from(stock: axum::Json<CreateStock>) -> Self {
        Self {
            symbol: stock.symbol.clone(),
            name: stock.name.clone(),
            exchange: stock.exchange.clone(),
            sector: stock.sector.clone(),
            industry: stock.industry.clone(),
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, sqlx::FromRow)]
struct StockPrice {
    price_id: i32,
    stock_id: i32,
    date: NaiveDateTime,
    open: BigDecimal,
    high: BigDecimal,
    low: BigDecimal,
    close: BigDecimal,
    adjusted_close: BigDecimal,
    volume: i32
}

#[derive(Debug, Clone, Deserialize, Serialize)]
struct StockPriceConvert {
    price_id: i32,
    stock_id: i32,
    date: NaiveDateTime,
    open: f64,
    high: f64,
    low: f64,
    close: f64,
    adjusted_close: f64,
    volume: i32,
}

impl StockPrice {
    fn to_convert(&self) -> StockPriceConvert {
        StockPriceConvert {
            price_id: self.price_id,
            stock_id: self.stock_id,
            date: self.date,
            open: self.open.to_f64().unwrap_or(0.0),
            high: self.high.to_f64().unwrap_or(0.0),
            low: self.low.to_f64().unwrap_or(0.0),
            close: self.close.to_f64().unwrap_or(0.0),
            adjusted_close: self.adjusted_close.to_f64().unwrap_or(0.0),
            volume: self.volume,
        }
    }
}


#[derive(Debug, Clone, Deserialize, Serialize, sqlx::FromRow)]
struct CreateStockPrice {
    stock_id: i32,
    date: String,
    open: f32,
    high: f32,
    low: f32,
    close: f32,
    adjusted_close: f32,
    volume: BigDecimal
}

#[derive(Debug, Deserialize)]
pub struct DateStr {
    start_date: String,
    end_date: String
}


/* Services */
async fn get_stock(
    State(db_pool): State<PgPool>,
    Path(symbol): Path<String>
) -> impl IntoResponse {
    let symbol = symbol.to_uppercase();
    let stock_row = sqlx::query_as!(
        Stock,
        "SELECT * FROM stocks WHERE symbol = $1",
        symbol
    )
    .fetch_one(&db_pool)
    .await
    .expect("Could not find stock");

    (StatusCode::OK, Json(stock_row))
}

async fn get_stocks(
    State(db_pool): State<PgPool>
) -> impl IntoResponse {
    let stock_rows = sqlx::query_as!(
        Stock,
        "SELECT * FROM stocks ORDER BY stock_id asc"
    )
    .fetch_all(&db_pool)
    .await
    .expect("Could find stock rows");

    (StatusCode::OK, Json(stock_rows))
}

async fn create_stock(
    State(db_pool): State<PgPool>,
    Json(payload): Json<CreateStock>
) -> impl IntoResponse {
    let stock_row = sqlx::query_as!(
        CreateStock,
        "INSERT into stocks (symbol, name, exchange, sector, industry)
            values($1, $2, $3, $4, $5) RETURNING
            symbol,
            name,
            exchange,
            sector,
            industry
        ",
        payload.symbol,
        payload.name,
        payload.exchange,
        payload.sector,
        payload.industry
    )
    .fetch_one(&db_pool)
    .await
    .expect("Could not create stock");

    (StatusCode::CREATED, Json(stock_row))
}

async fn get_stock_prices_by_stock(
    State(db_pool): State<PgPool>,
    Path(symbol): Path<String>,
    Query(date_param): Query<DateStr>
) -> impl IntoResponse {
    let symbol = symbol.to_uppercase();
    let stock_id = sqlx::query_as!(
        Stock,
        "SELECT * FROM stocks WHERE symbol = $1",
        symbol
    )
    .fetch_one(&db_pool)
    .await
    .expect("Could not find stock id")
    .stock_id;

    let stock_price_rows = sqlx::query_as!(
        StockPrice,
        "SELECT * FROM stock_prices WHERE stock_id = $1 ORDER BY date ASC",
        stock_id
    )
    .fetch_all(&db_pool)
    .await
    .expect("Could not find stock id");
    println!("{:?} {:?}", date_param.start_date, date_param.end_date);
    (StatusCode::OK, Json(stock_price_rows))
}


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Welcome to Stock service");
    dotenv().ok();
    let db_conn_str = std::env::var("DATABASE_URL").expect("Could not find DATABASE_URL");

    // set up connection pool
    let db_pool = PgPoolOptions::new()
        .max_connections(5)
        .acquire_timeout(Duration::from_secs(3))
        .connect(&db_conn_str)
        .await
        .expect("can't connect to database");

    let addr = SocketAddr::from(([127,0,0,1], 8000));

    let listener = tokio::net::TcpListener::bind(&addr)
        .await
        .expect("Error binding TcpListener");

    let app: Router = Router::new()
        .nest("/api", Router::new()
            .route("/hello", get(hello_route))
            .route("/stocks", get(get_stocks).post(create_stock))
            .route("/stocks/:symbol", get(get_stock))
            .route("/prices/:symbol", get(get_stock_prices_by_stock))
    ).with_state(db_pool);

    axum::serve(listener, app).await.expect("Error serving server");
    Ok(())
}


async fn hello_route() -> impl IntoResponse {
    (StatusCode::OK, Json("Welcome to Stock Service"))
}
