use axum::{Router, response::Json, routing::post};
use serde_json::Value;

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", post(save_stats));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn save_stats(Json(payload): Json<serde_json::Value>) -> Json<Value> {
    Json(payload)
}
