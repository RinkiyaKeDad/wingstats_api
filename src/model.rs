use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Database model for a player
#[derive(Debug, Deserialize, Serialize, sqlx::FromRow)]
pub struct PlayerModel {
    pub player_id: Uuid,
    pub name: String,
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
}
