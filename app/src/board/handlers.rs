use crustchan::dynamodb::{get_post_by_id, list_boards};
use crustchan::response::{GenericResponse, WebResult};
use tracing::info;

pub async fn get_post_by_id_handler(_board_id: String, post_id: String) -> WebResult {
    info!("get_post_by_id_handler:");
    let post = get_post_by_id(post_id).await.unwrap();

    let message = serde_json::to_string(&post).unwrap();

    let response = GenericResponse::new(warp::http::StatusCode::OK, message);
    Ok(response)
}

pub async fn get_boards_handler() -> WebResult {
    info!("get_boards_handler:");
    let boards = list_boards().await.unwrap();
    let string_with_escapes = serde_json::to_string(&boards).unwrap();

    let response = GenericResponse::new(warp::http::StatusCode::OK, string_with_escapes);
    Ok(response)
}
