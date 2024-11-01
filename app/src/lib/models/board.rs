use super::{deserialize_dt, serialize_dt};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct Board {
    pub id: String,
    pub name: String,
    pub description: String,
    pub sfw: bool,
    #[serde(serialize_with = "serialize_dt", deserialize_with = "deserialize_dt")]
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct BoardInput {
    pub name: String,
    pub description: String,
    pub sfw: bool,
}

impl Into<Board> for BoardInput {
    fn into(self) -> Board {
        Board {
            id: Uuid::new_v4().to_string(),
            name: self.name,
            description: self.description,
            sfw: self.sfw,
            created_at: chrono::offset::Utc::now(),
        }
    }
}
