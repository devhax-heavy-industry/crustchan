use super::{deserialize_dt, serialize_dt};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug, Serialize, Clone, Deserialize,ToSchema)]
pub struct Post {
    pub id: String,
    pub subject: String,
    pub text: String,
    pub board_id: String,
    pub poster: String,
    pub file: String,
    pub ip: String,
    pub deleted: bool,
    pub soft_banned: bool,
    pub approved: bool,
    pub rejected: bool,
    pub locked: bool,
    pub sticky: bool,
    pub public_banned: Option<String>,
    pub op: String,
    pub file_name: String,
    pub file_size: u64,
    pub file_dimensions: String,
    pub file_original_name: String,
    #[serde(serialize_with = "serialize_dt", deserialize_with = "deserialize_dt")]
    #[schema(value_type = String, format = "date-time")]
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Clone, Deserialize,ToSchema)]
pub struct PostInput {
    pub subject: String,
    pub text: String,
    pub board_id: String,
    pub poster: String,
    pub file: String,
    pub op: Option<String>,
    pub file_name: String,
    pub file_size: u64,
    pub file_dimensions: String,
    pub file_original_name: String,
}

impl From<PostInput> for Post {
    fn from(post: PostInput) -> Post {
        Post {
            id: Uuid::new_v4().to_string(),
            subject: post.subject,
            text: post.text,
            board_id: post.board_id,
            poster: post.poster,
            file: post.file,
            ip: "".to_string(),
            deleted: false,
            soft_banned: false,
            approved: false,
            rejected: false,
            locked: false,
            sticky: false,
            public_banned: None,
            op: "NULL".to_string(),
            file_name: post.file_name,
            file_size: post.file_size,
            file_dimensions: post.file_dimensions,
            file_original_name: post.file_original_name,
            created_at: chrono::offset::Utc::now(),
        }
    }

}

impl Default for Post {
    fn default() -> Post {
        Post {
            id: Uuid::new_v4().to_string(),
            subject: "".to_string(),
            text: "".to_string(),
            poster: "".to_string(),
            board_id: "".to_string(),
            ip: "".to_string(),
            file: "".to_string(),
            deleted: false,
            soft_banned: false,
            locked: false,
            approved: false,
            rejected: false,
            sticky: false,
            public_banned: None,
            op: "NULL".to_string(),
            file_name: "".to_string(),
            file_size: 0,
            file_dimensions: "".to_string(),
            file_original_name: "".to_string(),
            created_at: chrono::offset::Utc::now(),
        }
    }
}


#[derive(Debug, Serialize, Clone, Deserialize, ToSchema)]
pub struct FetchPostInput {
    id: String
}