use super::{deserialize_dt, serialize_dt};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone, ToSchema)]
pub struct Admin {
    pub id: String,
    pub username: String,
    pub password: String,
    #[serde(serialize_with = "serialize_dt", deserialize_with = "deserialize_dt")]
    pub created_at: DateTime<Utc>,
}

impl Default for Admin {
    fn default() -> Admin {
        Admin {
            id: Uuid::new_v4().to_string(),
            username: "".to_string(),
            password: "".to_string(),
            created_at: chrono::offset::Utc::now(),
        }
    }
}
