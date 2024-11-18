use super::{deserialize_dt, serialize_dt};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use utoipa::ToSchema;

#[derive(Debug, Serialize, Clone, Deserialize,ToSchema)]
pub struct Board {
    pub id: String,
    pub name: String,
    pub description: String,
    pub sfw: bool,
    #[serde(serialize_with = "serialize_dt", deserialize_with = "deserialize_dt")]
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Clone, Deserialize, ToSchema)]
pub struct BoardInput {
    pub name: String,
    pub description: String,
    pub sfw: bool,
}

impl From<BoardInput> for Board {
    fn from(board: BoardInput) -> Board {
        Board {
            id: Uuid::new_v4().to_string(),
            name: board.name,
            description: board.description,
            sfw: board.sfw,
            created_at: chrono::offset::Utc::now(),
        }
    }
}

