use std::sync::Arc;

use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use serde_json::json;
use uuid::Uuid;

use crate::{AppState, model::PlayerModel, schema::PlayerSchema};

pub async fn player_list_handler(
    State(data): State<Arc<AppState>>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    // Query with macro
    let players = sqlx::query_as!(PlayerModel, r#"SELECT * FROM players ORDER by player_id"#)
        .fetch_all(&data.db)
        .await
        .map_err(|e| {
            let error_response = serde_json::json!({
                "status": "error",
                "message": format!("Database error: { }", e),
            });
            (StatusCode::INTERNAL_SERVER_ERROR, Json(error_response))
        })?;

    let json_response = serde_json::json!({
        "status": "ok",
        "count": players.len(),
        "notes": players
    });

    Ok(Json(json_response))
}

pub async fn get_player_handler(
    Path(player_id): Path<Uuid>,
    State(data): State<Arc<AppState>>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let query_result = sqlx::query_as!(
        PlayerModel,
        r#"SELECT * FROM players WHERE player_id = $1"#,
        &player_id
    )
    .fetch_one(&data.db)
    .await;

    match query_result {
        Ok(player) => {
            let player_response = serde_json::json!({
                "status" : "success",
                "data": serde_json::json!({
                    "player": player
                })
            });

            Ok(Json(player_response))
        }
        Err(sqlx::Error::RowNotFound) => {
            let error_response = serde_json::json!({
                "status": "fail",
                "message": format!("Player with ID: {} not found", player_id)
            });
            Err((StatusCode::NOT_FOUND, Json(error_response)))
        }
        Err(e) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"status": "error", "message": format!("{:?}", e)})),
        )),
    }
}

pub async fn create_player_handler(
    State(data): State<Arc<AppState>>,
    Json(body): Json<PlayerSchema>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    // Insert
    let player_id = uuid::Uuid::new_v4();
    let player = sqlx::query_as!(
        PlayerModel,
        r#"INSERT INTO players (player_id, name) VALUES ($1, $2) RETURNING *"#,
        &player_id,
        &body.name
    )
    .fetch_one(&data.db)
    .await
    .map_err(|err: sqlx::Error| err.to_string());

    // Duplicate err check
    if let Err(err) = player {
        if err.to_string().contains("duplicate key value") {
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

    let player_response = json!({
            "status": "success",
            "data": json!({
                "player": player
        })
    });

    Ok(Json(player_response))
}

pub async fn delete_player_handler(
    Path(player_id): Path<Uuid>,
    State(data): State<Arc<AppState>>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    // Insert
    let query_result = sqlx::query(r#"DELETE FROM players WHERE player_id = $1"#)
        .bind(&player_id)
        .execute(&data.db)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "status": "error",
                    "message": format!("{:?}", e)
                })),
            )
        })?;

    if query_result.rows_affected() == 0 {
        let error_response = serde_json::json!({
            "status": "error",
            "message": format!("Player with ID: {} not found", player_id)
        });
        return Err((StatusCode::NOT_FOUND, Json(error_response)));
    }

    Ok(StatusCode::OK)
}
