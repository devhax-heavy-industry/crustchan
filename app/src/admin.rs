use crate::dynamodb::create_board;
use crate::GenericResponse;
use crate::WebResult;
use crate::CONTENT_LIMIT;
use tracing::info;
use std::collections::HashMap;
use bytes::Bytes;
use serde_dynamo::{to_item,from_item, Item,AttributeValue};
use warp::filters::BoxedFilter;
use warp::{Reply, Buf, Filter};
use serde_json::from_str;
use crate::model::Board;

pub fn admin_ban() -> BoxedFilter<(impl Reply,)> {
    warp::post()
        .and(warp::path("api"))
        .and(warp::path("admin"))
        .and(warp::path("ban"))
        .and(warp::body::content_length_limit(CONTENT_LIMIT))
        .and_then(ban_handler)
        .boxed()
}
pub fn admin_list_posts_by_session() -> BoxedFilter<(impl Reply,)> {
    warp::get()
        .and(warp::path("api"))
        .and(warp::path("admin"))
        .and(warp::path("posts-by-session"))
        .and_then(ban_handler)
        .boxed()
}

pub fn admin_list_posts_by_ip() -> BoxedFilter<(impl Reply,)> {
    warp::get()
        .and(warp::path!("api" / "admin" / "posts-by-ip"))
        .and_then(ban_handler)
        .boxed()
}

pub fn admin_routes() -> BoxedFilter<(impl Reply,)> {
    admin_ban().boxed()
        .or(admin_list_posts_by_session().boxed())
        .or(admin_list_posts_by_ip().boxed())
        .or(create_board_route().boxed())
        .boxed()
}

pub async fn ban_handler() -> WebResult<impl Reply> {
    info!("ban_handler:");
    const MESSAGE: &str = "lel ban em all";

    let response = GenericResponse::new(warp::http::StatusCode::OK, MESSAGE.to_string());
    info!("response: {:?}", response);
    Ok(response)
}

pub async fn create_board_handler(json_body:Board) -> WebResult<impl Reply> {
  info!("list_posts_by_board_handler:");
  // let item: Item = json_body.into();
  // let board = from_item::<Item, Board>(item).unwrap();
  // let serde_string: String =  json_body.serialize().unwrap();
  // let json_string = serde_json::to_string(&json_body).unwrap();
  // let board = from_str::<Board>(json_string.as_str());
  // let item = to_item(json_string)?;
  // let posts = create_board(board).await.unwrap();

  let msg = format!("json body: {json_body:?}");
  info!(msg);


  // let message: String = format!("{:?}", posts);

  let response = GenericResponse::new(warp::http::StatusCode::OK, msg);
  Ok(response)
}

pub fn create_board_route() -> BoxedFilter<(impl Reply,)> {
warp::post()
  .and(warp::path!("api" / "admin" / "board"))
  .and(warp::body::content_length_limit(CONTENT_LIMIT))
  .and(warp::body::json())
  .and_then(create_board_handler)
  .boxed()
}
