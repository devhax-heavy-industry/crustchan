use super::{deserialize_dt, serialize_dt};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use aws_lambda_events::s3::S3EventRecord; 

#[derive(Debug, Serialize, Clone, Deserialize)]
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
    pub locked: bool,
    pub sticky: bool,
    pub public_banned: Option<String>,
    pub op: String,
    pub file_name: String,
    pub file_size: u64,
    pub file_dimensions: String,
    pub file_original_name: String,
    #[serde(serialize_with = "serialize_dt", deserialize_with = "deserialize_dt")]
    pub created_at: DateTime<Utc>,
}
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

impl Into<Post> for PostInput {
    fn into(self) -> Post {
        Post {
            id: Uuid::new_v4().to_string(),
            subject: self.subject,
            text: self.text,
            board_id: self.board_id,
            poster: self.poster,
            file: self.file,
            ip: "".to_string(),
            deleted: false,
            soft_banned: false,
            approved: false,
            locked: false,
            sticky: false,
            public_banned: None,
            op: "NULL".to_string(),
            file_name: self.file_name,
            file_size: self.file_size,
            file_dimensions: self.file_dimensions,
            file_original_name: self.file_original_name,
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
