use axum::{routing::{get, post}, Router};
use dotenv::dotenv;
use std::env;
use std::net::SocketAddr;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL not found in .env file");
    let addr = SocketAddr::from(([127,0,0,1], 8000));

    let app = Router::new()
        .nest("/api", Router::new()
            .route("/greet", get(|| async {"Greetings"}))
    );

    let listener: TcpListener = tokio::net::TcpListener::bind(&addr)
        .await
        .expect("Error serving application");

    axum::serve(listener, app).await.unwrap();
    
    Ok(())
}