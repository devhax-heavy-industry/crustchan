use aws_lambda_events::dynamodb::EventRecord;
use serde::{Deserialize, Serialize};
use serde_dynamo::{from_item,Item};
use crate::models::Post;

#[derive(Debug, Deserialize, Serialize)]
#[serde(tag = "type")]
#[allow(clippy::large_enum_variant)]
pub enum PostEvent {
    Created { item: Post },
    Updated { old: Post, new: Post },
    Deleted { item: Post },
}


impl PostEvent {
    pub fn id(&self) -> &str {
        match self {
            PostEvent::Created { item } => item.id.as_str(),
            PostEvent::Updated { new, .. } => new.id.as_str(),
            PostEvent::Deleted { item } => item.id.as_str(),
        }
    }
}
impl TryFrom<EventRecord> for PostEvent {
  type Error = anyhow::Error;

  /// Try converting a DynamoDB record to an event.
  fn try_from(value: EventRecord) -> Result<Self, Self::Error> {
      match value.event_name.as_str() {
          "INSERT" => {
              let item = (&value.change.new_image).try_into()?;
              Ok(PostEvent::Created { item })
          }
          "MODIFY" => {
              let old = (&value.change.old_image).try_into()?;
              let new = (&value.change.new_image).try_into()?;
              Ok(PostEvent::Updated { old, new })
          }
          "REMOVE" => {
              let item = (&value.change.old_image).try_into()?;
              Ok(PostEvent::Deleted { item })
          }
          _ => Err(anyhow::anyhow!("Unknown event type: {}", value.event_name)),
      }
  }
}

impl TryFrom<&Item> for Post {
  type Error = anyhow::Error;

  fn try_from(value: &Item) -> Result<Self, Self::Error> {
    let post:Post = from_item(value.clone())?;
    Ok(post)
  }
}