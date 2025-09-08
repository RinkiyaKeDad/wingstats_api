use std::sync::Arc;

use axum::{Router, routing::get, routing::post};

use crate::{
    AppState, handler::create_player_handler, handler::get_player_handler,
    handler::player_list_handler,
};

pub fn create_router(app_state: Arc<AppState>) -> Router {
    Router::new()
        .route("/api/v1/players", post(create_player_handler))
        .route("/api/v1/players", get(player_list_handler))
        .route("/api/v1/players/{id}", get(get_player_handler))
        .with_state(app_state)
}
