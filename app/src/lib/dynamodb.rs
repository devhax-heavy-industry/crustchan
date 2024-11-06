use crate::models::{Admin, Board, Post};
use crate::rejections::InvalidUser;
use crate::AWS_REGION;
use rusoto_dynamodb::{
    AttributeValue, 
    DynamoDb, 
    DynamoDbClient, 
    PutItemInput, 
    UpdateItemInput, 
    UpdateItemOutput,
    PutItemOutput, 
    QueryInput, 
    ScanInput,
};
use serde::Deserialize;
use serde_dynamo::{from_item, to_item, Item};
use std::collections::HashMap;
use std::error::Error;
use tokio::sync::OnceCell;
use tracing::{info, warn};
use warp::Rejection;

pub const TABLES: &[&str] = &["posts", "boards", "admin"];

pub const POSTS_TABLE_NAME: &str = "crustchan-posts";
pub const BOARDS_TABLE_NAME: &str = "crustchan-boards";
pub const ADMIN_TABLE_NAME: &str = "crustchan-admin";

#[derive(Debug, Deserialize)]
pub struct QueryOptions {
    pub page: Option<usize>,
    pub limit: Option<usize>,
}

pub async fn get_client() -> &'static DynamoDbClient {
    static CLIENT: OnceCell<DynamoDbClient> = OnceCell::const_new();
    CLIENT
        .get_or_init(|| async {
            let client = DynamoDbClient::new(AWS_REGION);
            client
        })
        .await
}

pub async fn create_post(post: Post) -> Result<PutItemOutput, Box<dyn Error>> {
    let client: &DynamoDbClient = get_client().await;
    let item = to_item(post.clone())?;
    let input = PutItemInput {
        table_name: POSTS_TABLE_NAME.to_string(),
        item,
        ..Default::default()
    };

    let output = client.put_item(input).await?;

    Ok(output)
}

pub async fn list_posts_by_board(board: String) -> Result<Vec<Post>, Box<dyn Error>> {
    let client: &DynamoDbClient = get_client().await;
    warn!("Wow about to lists posts for board {}", board);
    let input = QueryInput {
        table_name: POSTS_TABLE_NAME.to_string(),
        index_name: Some("board-index".to_string()),
        key_condition_expression: Some("board_id = :inputboard".to_string()),
        expression_attribute_values: Some(
            [(
                ":inputboard".to_string(),
                AttributeValue {
                    s: Some(board),
                    ..Default::default()
                },
            )]
            .iter()
            .cloned()
            .collect(),
        ),
        ..Default::default()
    };

    let output = client.query(input).await?;

    let items = output.items.unwrap();
    let posts: Vec<Post> = items
        .iter()
        .map(|item| from_item(item.clone()).unwrap())
        .collect();
    Ok(posts)
}

pub async fn create_board(board: Board) -> Result<PutItemOutput, Box<dyn Error>> {
    let client: &DynamoDbClient = get_client().await;
    let item = to_item(board.clone())?;

    let input: PutItemInput = PutItemInput {
        table_name: BOARDS_TABLE_NAME.to_string(),
        item,
        ..Default::default()
    };

    let output = client.put_item(input).await?;
    info!("Created board item: {:?}", output.clone());
    Ok(output)
}
pub async fn get_post_by_id(post_id: String) -> Result<Post, Box<dyn Error>> {
    let client: &DynamoDbClient = get_client().await;
    let input = QueryInput {
        table_name: POSTS_TABLE_NAME.to_string(),
        key_condition_expression: Some(
            "id = :inputpost".to_string(),
        ),
        expression_attribute_values: Some(
            [
                (
                    ":inputpost".to_string(),
                    AttributeValue {
                        s: Some(post_id),
                        ..Default::default()
                    },
                ),
                // (
                //     ":inputcreated_at".to_string(),
                //     AttributeValue {
                //         s: Some("00000".to_string()),
                //         ..Default::default()
                //     },
                // ),
            ]
            .iter()
            .cloned()
            .collect(),
        ),
        ..Default::default()
    };
    let output = client.query(input).await?;

    let item = output.items.unwrap().pop().unwrap();
    let post: Post = from_item(item).unwrap();

    info!("get_post_by_id - Post: {:?}", post.clone());
    Ok(post)
}

pub async fn list_boards() -> Result<Vec<Board>, Box<dyn Error>> {
    let client: &DynamoDbClient = get_client().await;
    let output = client
        .scan(ScanInput {
            table_name: BOARDS_TABLE_NAME.to_string(),
            ..Default::default()
        })
        .await?;
    let items = output.items.unwrap().to_vec();

    info!("list_boards - ITEMS: {:?}", items.clone());
    let boards: Vec<Board> = items
        .iter()
        .map(|item| from_item(item.clone()).unwrap())
        .collect();
    info!("list_boards - BOARDS: {:?}", boards.clone());
    Ok(boards)
}

pub async fn get_admin_user(username: String) -> Result<Admin, Rejection> {
    info!("get_admin_user - Username: {:?}", username);
    let client: &DynamoDbClient = get_client().await;
    let input = QueryInput {
        table_name: ADMIN_TABLE_NAME.to_string(),
        key_condition_expression: Some("username = :inputname".to_string()),
        index_name: Some("username-index".to_string()),
        expression_attribute_values: Some(
            [(
                ":inputname".to_string(),
                AttributeValue {
                    s: Some(username),
                    ..Default::default()
                },
            )]
            .iter()
            .cloned()
            .collect(),
        ),
        ..Default::default()
    };
    let output = client.query(input).await;
   
    let items_output = match output {
        Err(e) => {
            warn!("get_admin_user - Error: {:?}", e);
            return Err::<_, Rejection>(warp::reject::custom(InvalidUser));
        }
        _ => output.unwrap(),
    };
    info!("username yielded an admin user");
    let user: Admin = from_item(items_output.items.unwrap().pop().unwrap()).unwrap();
    Ok(user)
}

pub async fn get_any_admin_user() -> Result<Admin, Rejection> {
    let client: &DynamoDbClient = get_client().await;
    let input = ScanInput {
        table_name: ADMIN_TABLE_NAME.to_string(),
        limit: Some(1),
        ..Default::default()
    };
    let output = client.scan(input).await;
    let items_output = match output {
        Err(e) => {
            warn!("get_any_admin_user - Error: {:?}", e);
            return Err::<_, Rejection>(warp::reject::custom(InvalidUser));
        }
        _ => output.unwrap(),
    };
    let items = items_output.items.unwrap().pop();
    if items.is_none() {
        return Err::<_, Rejection>(warp::reject::custom(InvalidUser));
    } else {
        let user: Admin = from_item(items.unwrap()).unwrap();
        Ok(user)
    }
}

pub async fn create_admin(admin: Admin) -> Result<PutItemOutput, Box<dyn Error>> {
    let client: &DynamoDbClient = get_client().await;
    let item = to_item(admin.clone())?;

    let input: PutItemInput = PutItemInput {
        table_name: ADMIN_TABLE_NAME.to_string(),
        item,
        ..Default::default()
    };

    let output = client.put_item(input).await?;
    info!("Created admin item: {:?}", output.clone());
    Ok(output)
}

pub async fn update_post(post: Post) -> Result<PutItemOutput, Box<dyn Error>> {
    let client: &DynamoDbClient = get_client().await;
    let item = to_item(post.clone())?;

    let input = PutItemInput {
        table_name: POSTS_TABLE_NAME.to_string(),
        item,
        ..Default::default()
    };

    let output = client.put_item(input).await?;

    Ok(output)
}

pub async fn approve_post(post_id:String) -> Result<PutItemOutput, Box<dyn Error>> {
    info!("Fetching post by id {}", post_id.clone());
    let mut post = get_post_by_id(post_id.clone()).await?;
    info!("Post fetched: {:?}", post.clone());
    post.approved = true;
    post.rejected = false;
    info!("Updating post");
    let output = update_post(post).await?;
    Ok(output)
}


pub async fn reject_post(post_id:String) -> Result<PutItemOutput, Box<dyn Error>> {
    info!("Fetching post by id {}", post_id.clone());
    let mut post = get_post_by_id(post_id.clone()).await?;
    info!("Post fetched: {:?}", post.clone());
    post.approved = false;
    post.rejected = true;
    info!("Updating post");
    let output = update_post(post).await?;
    Ok(output)
}