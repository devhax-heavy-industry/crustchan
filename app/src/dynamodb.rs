
    use aws_config::SdkConfig;
    use aws_sdk_dynamodb::{Client};
    use aws_sdk_dynamodb::types::{AttributeValue};
    use serde_dynamo::{to_item,Item};
    use tracing::info;
    use std::error::Error;
    use tokio::sync::OnceCell;
    use crate::model::Post;

    pub const TABLES: &[&str] = &["posts", "boards", "admin"];

    pub const POSTS_TABLE_NAME: &str = "posts";

    pub async fn get_client() -> &'static Client {
        static CLIENT: OnceCell<Client> = OnceCell::const_new();
        CLIENT.get_or_init( || async {
            let shared_config = aws_config::load_from_env().await;
            let client = Client::new(&shared_config);
            client
        }).await
    }

    

    // get a list of all tables in dynamodb
    pub async fn list_tables() -> Result<Vec<String>, Box<dyn Error>> {
      let client:&Client = get_client().await;
        let paginator = client.list_tables().into_paginator().items().send();
        let table_names = paginator.collect::<Result<Vec<_>, _>>().await?;
        Ok(table_names)
    }

    pub async fn create_post(post: Post) -> Result<Post, Box<dyn Error>> {
        let client:&Client = get_client().await;
        let item:Item = to_item(post.clone()).unwrap();
        let req = client
            .put_item()
            .table_name(POSTS_TABLE_NAME)
            .set_item(Some(item))
            .send()
            .await?;
        Ok(req)
    }

    pub async fn list_posts_by_board(board: String) -> Result<Vec<Post>, Box<dyn Error>> {
      let client:&Client = get_client().await;
        let req = client
            .query()
            .table_name(POSTS_TABLE_NAME)
            .key_condition_expression("board =  :inputboard")
            .expression_attribute_values(":inputboard", AttributeValue::S(board))
            .send()
            .await?;

        info!("list_posts_by_board: {:?}", req);
        Ok(())
    }
