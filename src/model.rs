use serde::{Deserialize, Serialize};

/// Database model for a note
#[derive(Debug, Deserialize, Serialize, sqlx::FromRow)]
pub struct PlayerModel {
    pub player_id: String,
    pub name: String,
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
}
