use crate::auth::{AuthnToken,login};
use crustchan::dynamodb::create_board;
use crustchan::models::Board;
use crustchan::rejections::InvalidUser;
use crustchan::response::{GenericResponse, WebResult};
use std::collections::HashMap;
use tracing::{error, info};
use warp::reply::WithHeader;
use warp::Rejection;
use warp::Reply;

pub async fn ban_handler(_token: impl Reply) -> WebResult {
    info!("ban_handler:");
    const MESSAGE: &str = "lel ban em all";

    let response = GenericResponse::new(warp::http::StatusCode::OK, MESSAGE.to_string());
    info!("response: {:?}", response);
    Ok(response)
}

pub async fn create_board_handler(
    _token: AuthnToken,
    json_body: HashMap<String, String>,
) -> WebResult {
    info!("create_board_handler:");
    let json_str = serde_json::to_string(&json_body).unwrap();
    let board: Board = serde_json::from_str(&json_str).unwrap();
    let __db_board = create_board(board.clone()).await.unwrap();
    let message: String = format!("board: {:?}", board.clone());

    let response = GenericResponse::new(warp::http::StatusCode::OK, message);
    Ok(response)
}

pub async fn admin_lists_posts_by_ip_handler(_token: impl Reply) -> WebResult {
    info!("admin_lists_posts_by_ip_handler:");
    let posts = vec!["post1", "post2", "post3"];
    let json_string = serde_json::to_string(&posts).unwrap();
    let response = GenericResponse::new(warp::http::StatusCode::OK, json_string);
    Ok(response)
}

pub async fn login_handler(
    json_body: HashMap<String, String>,
) -> Result<WithHeader<GenericResponse>, Rejection> {
    info!("login_handler:");
    let username = json_body.get("username").unwrap();
    let password = json_body.get("password").unwrap();
    let result = login(username.clone(), password.clone())
        .await
        .map_err(|e| {
            error!("login error: {:?}", e);
            let _ = warp::reject::custom(InvalidUser);
        })
        .unwrap();
    let user_id = result.id.parse::<i64>().unwrap();
    let token = AuthnToken::from_user_id(user_id).unwrap();
    let msg = format!("Welcome back, {}", result.username);
    Ok(warp::reply::with_header(
        GenericResponse::new(warp::http::StatusCode::OK, msg),
        "Set-Cookie",
        token.header_val(),
    ))
}
