use super::{deserialize_dt, serialize_dt};
use crate::rejections::{HashError, InvalidLogin};
use argonautica::Hasher;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use warp::Rejection;
use tokio::sync::OnceCell;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Admin {
    pub id: String,
    pub username: String,
    pub password: String,
    #[serde(serialize_with = "serialize_dt", deserialize_with = "deserialize_dt")]
    pub created_at: DateTime<Utc>,
}
impl<'a> Admin{
    pub async fn get_secret_key() -> &'static String {
        static SECRET_KEY: OnceCell<String> = OnceCell::const_new();
        SECRET_KEY
            .get_or_init(|| async {
                let secret_key = std::env::var("SECRET_KEY").unwrap_or_else(|_| {
                    "f7sigsef esf fh2dsjrd k56fg fshdj4g,fhjd6we easfra sfda2kg".repeat(8)
                });
                secret_key
            })
            .await
    }

    pub async fn hash_password(password: String) -> Result<String, HashError> {
        let secret = Admin::get_secret_key().await;
        Ok(Hasher::default()
            .with_password(password)
            .with_secret_key(secret.as_str())
            .hash().unwrap())
    }
    pub async fn login(username: String, password: String) -> Result<Admin, Rejection> {
        let admin: Admin = crate::dynamodb::get_admin_user(username.clone())
            .await
            .unwrap();
        let hashed_password = Admin::hash_password(password).await.unwrap();
        if admin.username == username && admin.password == hashed_password {
            return Ok(admin);
        } else {
            return Err(warp::reject::custom(InvalidLogin));
        }
    }
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
