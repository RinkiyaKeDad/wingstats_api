use serde::{Deserialize, Serialize};

/// Schema for creating or updating a player
#[derive(Serialize, Deserialize, Debug)]
pub struct PlayerSchema {
    pub name: String,
}
