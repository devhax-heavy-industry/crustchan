pub mod board;
pub mod post;
pub mod admin;

pub use board::*;
pub use post::*;
pub use admin::*;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize, Serializer, Deserializer};


pub fn serialize_dt<S>(dt: &DateTime<Utc>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
  let serialized:Result<<S>::Ok, <S>::Error> = dt.to_rfc3339().to_string().serialize(serializer);
  serialized
}

pub fn deserialize_dt<'de, D>(deserializer: D) -> Result<DateTime<Utc>, D::Error>
where
    D: Deserializer<'de>
    {
      
        let s = String::deserialize(deserializer)?;
        DateTime::parse_from_rfc3339(&s)
            .map_err(serde::de::Error::custom)
            .map(DateTime::from)
    }







