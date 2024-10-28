    use rusoto_core::Region;
    use rusoto_dynamodb::{AttributeValue, DynamoDb, DynamoDbClient, PutItemInput, PutItemOutput, QueryInput, ScanInput};
    use serde_dynamo::{to_item, from_items, from_item, Item};
    use tracing::{info, warn};
    use std::error::Error;
    use tokio::sync::OnceCell;
    use crate::model::{Post,Board};

    pub const TABLES: &[&str] = &["posts", "boards", "admin"];

    pub const POSTS_TABLE_NAME: &str = "crustchan-posts";
    pub const BOARDS_TABLE_NAME: &str = "crustchan-boards";
    pub const ADMIN_TABLE_NAME: &str = "crustchan-admin";

    pub async fn get_client() -> &'static DynamoDbClient {
        static CLIENT: OnceCell<DynamoDbClient> = OnceCell::const_new();
        CLIENT.get_or_init( || async {
          let client = DynamoDbClient::new(Region::UsWest2);
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
      warn!("Wow about to lists posts for board {}", board);
      let input = QueryInput {
        table_name: POSTS_TABLE_NAME.to_string(),
        index_name: Some("board-index".to_string()),
        key_condition_expression: Some("board_id = :inputboard".to_string()),
        expression_attribute_values: Some(
          [(":inputboard".to_string(), AttributeValue {
            s: Some(board),
            ..Default::default()
          })].iter().cloned().collect()
        ),
        ..Default::default()
      };

        let output = client
            .query(input)
            .await?;

        let items = output.items.unwrap();
        let posts: Vec<Post> = items.iter().map(|item| from_item(item.clone()).unwrap()).collect();
        Ok(posts)
    }

    pub async fn create_board(board: Board) -> Result<PutItemOutput, Box<dyn Error>> {
      let client:&DynamoDbClient = get_client().await;
      let item = to_item(board.clone())?;

      let input: PutItemInput = PutItemInput {
        table_name: BOARDS_TABLE_NAME.to_string(),
        item,
        ..Default::default()
       };

      let output = client
          .put_item(input)
          .await?;
       info!("Created board item: {:?}", output.clone());
      Ok(output)
  }
  pub async fn get_post_by_id(board_id:String, post_id:String) -> Result<Post, Box<dyn Error>> {
    let client:&DynamoDbClient = get_client().await;
    let input = QueryInput {
      table_name: POSTS_TABLE_NAME.to_string(),
      key_condition_expression: Some("id = :inputpost AND created_at > :inputcreated_at".to_string()),
      expression_attribute_values: Some(
        [
        (":inputpost".to_string(), AttributeValue {
          s: Some(post_id),
          ..Default::default()
        }),
        (":inputcreated_at".to_string(), AttributeValue {
          s: Some("00000".to_string()),
          ..Default::default()
        })].iter().cloned().collect()
        ),
      ..Default::default()
    };
    let output = client
        .query(input)
        .await?;

    let item = output.items.unwrap().pop().unwrap();

    let post: Post = from_item(item)?;

    info!("get_post_by_id - Post: {:?}", post.clone());
    Ok(post)
  }

  pub async fn list_boards()-> Result<Vec<Board>, Box<dyn Error>> {
    let client:&DynamoDbClient = get_client().await;
    let output = client
        .scan(ScanInput {
          table_name: BOARDS_TABLE_NAME.to_string(),
          ..Default::default()
        })
        .await?;
    let items = output.items.unwrap().to_vec();

    info!("list_boards - ITEMS: {:?}", items.clone());
    let boards: Vec<Board> = items.iter().map(|item| from_item(item.clone()).unwrap()).collect();
    info!("list_boards - BOARDS: {:?}", boards.clone());
    Ok(boards)
  }