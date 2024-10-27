    use rusoto_core::Region;
    use rusoto_dynamodb::{DynamoDb, AttributeValue, DynamoDbClient, PutItemInput, PutItemOutput, QueryInput};
    use serde_dynamo::{to_item, from_items};
    use tracing::info;
    use std::error::Error;
    use tokio::sync::OnceCell;
    use crate::model::{Post,Board};

    pub const TABLES: &[&str] = &["posts", "boards", "admin"];

    pub const POSTS_TABLE_NAME: &str = "posts";

    pub async fn get_client() -> &'static DynamoDbClient {
        static CLIENT: OnceCell<DynamoDbClient> = OnceCell::const_new();
        CLIENT.get_or_init( || async {
          let client = DynamoDbClient::new(Region::UsWest1);
            client
        }).await
    }

    

    // get a list of all tables in dynamodb
    // pub async fn list_tables() -> Result<Vec<String>, Box<dyn Error>> {
    //   let client:&DynamoDbClient = get_client().await;
    //     let paginator = client.list_tables().into_paginator().items().send();
    //     let table_names = paginator.collect::<Result<Vec<_>, _>>().await?;
    //     Ok(table_names)
    // }

    pub async fn create_post(post: Post) -> Result<PutItemOutput, Box<dyn Error>> {
        let client:&DynamoDbClient = get_client().await;
        let item = to_item(post.clone())?;

        let input = PutItemInput {
          table_name: POSTS_TABLE_NAME.to_string(),
          item,
          ..Default::default()
         };

        let output = client
            .put_item(input)
            .await?;

        Ok(output)
    }

    pub async fn list_posts_by_board(board: String) -> Result<Vec<Post>, Box<dyn Error>> {
      let client:&DynamoDbClient = get_client().await;
      let input = QueryInput {
        table_name: POSTS_TABLE_NAME.to_string(),
        key_condition_expression: Some("board_id = :inputboard".to_string()),
        expression_attribute_values: Some(
          [(":inputboard".to_string(), AttributeValue {
            s: Some(board),
            ..Default::default()
          })].iter().cloned().collect()
        ),
        ..Default::default()
      };
      /*
            .table_name(POSTS_TABLE_NAME)
            .key_condition_expression("board =  :inputboard")
            .expression_attribute_values(":inputboard", AttributeValue::S(board))
            .send() */
        let output = client
            .query(input)
            .await?;

        info!("list_posts_by_board: {:?}", output.clone());
        let posts: Vec<Post> = from_items(output.items.unwrap())?;
        // let user: User = from_item(item)?;
        Ok(posts)
    }

    pub async fn create_board(board: Board) -> Result<PutItemOutput, Box<dyn Error>> {
      let client:&DynamoDbClient = get_client().await;
      let item = to_item(board.clone())?;

      let input = PutItemInput {
        table_name: POSTS_TABLE_NAME.to_string(),
        item,
        ..Default::default()
       };

      let output = client
          .put_item(input)
          .await?;

      Ok(output)
  }