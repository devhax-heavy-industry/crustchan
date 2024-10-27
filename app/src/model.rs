use chrono::{DateTime, Utc};
use uuid::Uuid;
use serde::{Deserialize, Serialize, Serializer};
use aws_sdk_dynamodb::types::AttributeValue;
use std::collections::HashMap;

#[derive(Debug, Serialize, Clone,Deserialize)]
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
          board_id: "".to_string(),
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

impl Into<HashMap<std::string::String, aws_sdk_dynamodb::types::AttributeValue>> for Post {
  fn into(self) -> HashMap<std::string::String, aws_sdk_dynamodb::types::AttributeValue> {
    let mut item = HashMap::new();
    item.insert("subject".to_string(), AttributeValue::S(self.subject));
    item.insert("text".to_string(), AttributeValue::S(self.text));
    item.insert("poster".to_string(), AttributeValue::S(self.poster) );
    item.insert("board_id".to_string(), AttributeValue::S(self.board_id));
    item.insert("ip".to_string(), AttributeValue::S(self.ip));
    item.insert("file".to_string(), AttributeValue::S(self.file));
    item.insert("deleted".to_string(), AttributeValue::Bool(self.deleted));
    item.insert("soft_banned".to_string(), AttributeValue::Bool(self.soft_banned) );
    item.insert("locked".to_string(), AttributeValue::Bool(self.locked) );
    item.insert("approved".to_string(), AttributeValue::Bool(self.approved));
    item.insert("sticky".to_string(), AttributeValue::Bool(self.sticky) );
    item.insert("public_banned".to_string(), AttributeValue::S(if self.public_banned.is_none(){"".to_string()} else { self.public_banned.unwrap()} ));
    item.insert("op".to_string(), AttributeValue::S(self.op) );
    item.insert("file_name".to_string(), AttributeValue::S(self.file_name) );
    item.insert("file_size".to_string(), AttributeValue::N(self.file_size.to_string()) );
    item.insert("file_dimensions".to_string(), AttributeValue::S(self.file_dimensions) );
    item.insert("file_original_name".to_string(), AttributeValue::S(self.file_original_name) );
    item.insert("created_at".to_string(), AttributeValue::S(self.created_at.to_string()) );
    item
  }
}

#[derive(Debug, Serialize, Clone,Deserialize)]
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
  pub id: String,
  pub username: String,
  pub password: String,
  #[serde(serialize_with = "serialize_dt")]
  pub created_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct QueryOptions {
    pub page: Option<usize>,
    pub limit: Option<usize>,
}
