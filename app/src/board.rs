use crate::{GenericResponse, WebResult, CONTENT_LIMIT};
use crate::dynamodb::{get_post_by_id, list_boards};
use warp::filters::BoxedFilter;
use warp::{Filter, Reply};
use tracing::info;
use unescape::unescape;

pub fn get_board_by_id_route() -> BoxedFilter<(impl Reply,)> {
  warp::get()
    .and(warp::path!("api" / "board" / String / String))
    .and_then(get_post_by_id_handler)
    .boxed()
}

pub fn get_boards() -> BoxedFilter<(impl Reply,)> {
  warp::get()
    .and(warp::path!("api" / "board" ))
    .and_then(get_boards_handler)
    .boxed()
}


pub fn board_routes() -> BoxedFilter<(impl Reply,)> {
  get_board_by_id_route().or(get_boards()).boxed()
}

pub async fn get_post_by_id_handler(board_id:String, post_id:String) -> WebResult<impl Reply> {
  info!("get_post_by_id_handler:");
  let post = get_post_by_id(board_id, post_id).await.unwrap();

  let message = serde_json::to_string(&post).unwrap();

  let response = GenericResponse::new(warp::http::StatusCode::OK, message);
  Ok(response)
}

pub async fn get_boards_handler() -> WebResult<impl Reply> {
  info!("get_boards_handler:");
  let boards = list_boards().await.unwrap();
  let string_with_escapes = serde_json::to_string(&boards).unwrap();

  let response = GenericResponse::new(warp::http::StatusCode::OK, string_with_escapes);
  Ok(response)
}


