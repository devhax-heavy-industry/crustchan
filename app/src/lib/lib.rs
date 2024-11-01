pub mod models;
pub mod rejections;
pub mod response;
pub mod dynamodb;
pub mod utils;
use rusoto_core::Region;

pub const AWS_REGION:Region = Region::UsWest2;

pub const CONTENT_LIMIT: u64 = 1024 * 1024 * 25; // 25 MB