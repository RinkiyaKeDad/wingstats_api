use std::sync::Arc;

use axum::{Router, routing::post};

use crate::{AppState, handler::create_player_handler};

pub fn create_router(app_state: Arc<AppState>) -> Router {
    Router::new()
        .route("/api/v1/players", post(create_player_handler))
        .with_state(app_state)
}
