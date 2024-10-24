use chrono::{DateTime, Utc};
use uuid::Uuid;
use serde::{Deserialize, Serialize, Serializer};

#[derive(Debug, Serialize, Clone)]
pub struct Post {
  pub id: String,
  pub subject: String,
  pub text: String,
  pub board: String,
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
  #[serde(serialize_with = "serialize_dt")]
  pub created_at: DateTime<Utc>,
}

pub fn serialize_dt<S>(dt: &DateTime<Utc>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
  dt.format("%m/%d/%Y %H:%M")
    .to_string()
    .serialize(serializer)
}

impl Default for Post {
  fn default() -> Post {
    Post {
          id: Uuid::new_v4().to_string(),
          subject: "".to_string(),
          text: "".to_string(),
          poster: "".to_string(),
          board: "".to_string(),
          ip: "".to_string(),
          file: "".to_string(),
          deleted: false,
          soft_banned: false,
          locked: false,
          approved: false,
          sticky: false,
          public_banned: None,
          op: "".to_string(),
          file_name: "".to_string(),
          file_size: 0,
          file_dimensions: "".to_string(),
          file_original_name: "".to_string(),
          created_at: chrono::offset::Utc::now(),
      }
  }
}

#[derive(Debug, Serialize)]
pub struct Board {
  pub id: String,
  pub name: String,
  pub description: String,
  pub sfw: bool,
  #[serde(serialize_with = "serialize_dt")]
  pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize)]
pub struct Admin {
  pub username: String,
  pub passwprd: String,
  #[serde(serialize_with = "serialize_dt")]
  pub created_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct QueryOptions {
    pub page: Option<usize>,
    pub limit: Option<usize>,
}
