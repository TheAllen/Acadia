use std::{net::SocketAddr, time::Duration};
use axum::{
    extract::{Path, State}, 
    http::StatusCode, 
    response::IntoResponse, 
    routing::{get, post, put, delete}, 
    Json, Router
};
use chrono::{DateTime, Utc};
use dotenv::dotenv;
use serde::{Deserialize, Serialize};
use sqlx::{postgres::PgPoolOptions, PgPool};

/* Entities */ 
#[derive(Debug, Clone, Deserialize, Serialize, sqlx::FromRow)]
struct Bookmark {
    pub id: i32,
    pub title: String,
    pub url: String,
    pub description: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
struct CreateBookmark {
    pub title: String,
    pub url: String,
    pub description: Option<String>,
}

/* Services */
async fn get_bookmarks(State(db_pool): State<PgPool>) -> impl IntoResponse {
    let bookmark_rows = sqlx::query_as!(
        Bookmark,
        "SELECT * FROM bookmarks ORDER BY id ASC"
    )
    .fetch_all(&db_pool)
    .await
    .expect("Could find bookmark rows");

    (StatusCode::OK, Json(bookmark_rows))
}

async fn create_bookmark(
    State(db_pool): State<PgPool>,
    Json(payload): Json<CreateBookmark>
) -> impl IntoResponse {
    let bookmark_row = sqlx::query_as!(
        Bookmark,
        "INSERT INTO bookmarks (title, url, description, created_at)
            VALUES ($1, $2, $3, $4) RETURNING *
        ",
        payload.title,
        payload.url,
        payload.description,
        Utc::now()
    )
    .fetch_one(&db_pool)
    .await
    .expect("Could not create bookmark");

    (StatusCode::CREATED, Json(bookmark_row))
}

async fn update_bookmark(
    State(db_pool): State<PgPool>,
    Path(id): Path<i32>,
    Json(payload): Json<CreateBookmark>
) -> impl IntoResponse {
    let bookmark_row = sqlx::query_as!(
        Bookmark,
        "UPDATE bookmarks
            SET title = $1, url = $2, description = $3
            WHERE id = $4 RETURNING *
        ",
        payload.title,
        payload.url,
        payload.description,
        id
    )
    .fetch_one(&db_pool)
    .await
    .expect("Could not update bookmark");

    (StatusCode::OK, Json(bookmark_row))
}

async fn delete_bookmark(
    State(db_pool): State<PgPool>,
    Path(id): Path<i32>
) -> impl IntoResponse {
    sqlx::query!(
        "DELETE FROM bookmarks WHERE id = $1",
        id
    )
    .execute(&db_pool)
    .await
    .expect("Could not delete bookmark");

    (StatusCode::NO_CONTENT,)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Welcome to Bookmark Service");
    dotenv().ok();
    let db_conn_str = std::env::var("DATABASE_URL").expect("Could not find DATABASE_URL");

    // set up connection pool
    let db_pool = PgPoolOptions::new()
        .max_connections(5)
        .acquire_timeout(Duration::from_secs(3))
        .connect(&db_conn_str)
        .await
        .expect("can't connect to database");

    let app: Router = Router::new()
        .nest("/api", Router::new()
            .route("/bookmarks", get(get_bookmarks).post(create_bookmark))
            .route("/bookmarks/:id", put(update_bookmark).delete(delete_bookmark))
        )
        .with_state(db_pool);

    let addr = SocketAddr::from(([127, 0, 0, 1], 8000));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .expect("Error serving server");

    Ok(())
}