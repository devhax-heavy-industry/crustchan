use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::Mutex;

pub struct Post {
  pub id: String,
  pub subject: String,
  pub text: String,
  pub board: String,
  pub poster: String,
  pub image: String,
  pub file: String,
  pub ip: String,
  pub deleted: Boolean,
  pub soft_banned: Boolean,
  pub locked: Boolean,
  pub sticky: Boolean,
  pub public_banned: String,
  pub op: String,
  pub file_name: String,
  pub file_size: String,
  pub file_dimensions: String,
  pub file_original_name: String,
  pub createdAt: Option<DateTime<Utc>>,
}

pub struct Board {
  pub id: String,
  pub name: String,
  pub description: String,
  pub sfw: Boolean,
}

#[derive(Debug, Deserialize)]
pub struct QueryOptions {
    pub page: Option<usize>,
    pub limit: Option<usize>,
}