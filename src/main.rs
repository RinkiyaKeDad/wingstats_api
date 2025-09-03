use axum::{Router, response::Json, routing::post};
use dotenv::dotenv;
use serde_json::Value;
use sqlx::mysql::{MySqlPool, MySqlPoolOptions};
use std::sync::Arc;

pub struct AppState {
    db: MySqlPool,
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = match MySqlPoolOptions::new()
        .max_connections(10)
        .connect(&db_url)
        .await
    {
        Ok(pool) => {
            println!("Connected to DB successfully");
            pool
        }
        Err(err) => {
            println!("Failed to connect to DB: {}", err);
            std::process::exit(1);
        }
    };

    let app = Router::new()
        .route("/", post(save_stats))
        .with_state(Arc::new(AppState { db: pool.clone() }));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn save_stats(Json(payload): Json<serde_json::Value>) -> Json<Value> {
    Json(payload)
}
