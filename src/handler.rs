use std::sync::Arc;

use axum::{
    Json,
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
};
use serde_json::json;

use crate::{AppState, model::PlayerModel, schema::PlayerSchema};

// ...
pub async fn create_player_handler(
    State(data): State<Arc<AppState>>,
    Json(body): Json<PlayerSchema>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    // Insert
    let player_id = uuid::Uuid::new_v4().to_string();
    let query_result = sqlx::query(r#"INSERT INTO players (player_id, name) VALUES (?, ?)"#)
        .bind(&player_id)
        .bind(&body.name)
        .execute(&data.db)
        .await
        .map_err(|err: sqlx::Error| err.to_string());

    // Duplicate err check
    if let Err(err) = query_result {
        if err.contains("Duplicate entry") {
            let error_response = serde_json::json!({
                "status": "error",
                "message": "Player already exists",
            });
            return Err((StatusCode::CONFLICT, Json(error_response)));
        }

        return Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"status": "error","message": format!("{:?}", err)})),
        ));
    }

    // Get inserted note by ID with query macro
    let player = sqlx::query_as!(
        PlayerModel,
        r#"SELECT * FROM players WHERE player_id = ?"#,
        &player_id
    )
    .fetch_one(&data.db)
    .await
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"status": "error","message": format!("{:?}", e)})),
        )
    })?;

    let player_response = json!({
            "status": "success",
            "data": json!({
                "player": player
        })
    });

    Ok(Json(player_response))
}
