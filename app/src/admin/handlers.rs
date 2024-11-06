use crate::auth::{AuthnToken,login};
use crustchan::dynamodb::{approve_post, create_board, reject_post};
use crustchan::models::Board;
use crustchan::rejections::{InvalidUser, InvalidPost};
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
        });
    match result {
        Ok(_) => info!("login success"),
        Err(e) => {
            error!("login failed {:?}", e);
            let rejection = warp::reject::custom(InvalidUser);
            return Err(rejection);
        }
    };
    let actual_user = result.unwrap();
    let user_id = actual_user.id;
    let token = AuthnToken::from_user_id(user_id);

    match token {
        Ok(token_result) => {
            let msg = format!("Welcome back, {}", actual_user.username);
            Ok(warp::reply::with_header(
                GenericResponse::new(warp::http::StatusCode::OK, msg),
                "Set-Cookie",
                token_result.header_val(),
            ))
        }
        Err(_) => {
            error!("Token verification failed");
            let rejection = warp::reject::custom(InvalidUser);
            Err(rejection)
        }
        
    }

}



pub async fn approve_post_handler(_token: impl Reply, json_body: HashMap<String, String>) -> WebResult {
    info!("approve_post_handler:");
    let post_id = json_body.get("id").unwrap();

    let output = approve_post(post_id.clone()).await;
    match output {
        Ok(_) => {
            const MESSAGE: &str = "lel approve em all";

            let response = GenericResponse::new(warp::http::StatusCode::OK, MESSAGE.to_string());
            info!("response: {:?}", response);
            Ok(response)
        }
        Err(e) => {
            error!("approve_post_handler error: {:?}", e);
            let rejection = warp::reject::custom(InvalidPost);
            Err(rejection)
        }
    }


}

pub async fn reject_post_handler(_token: impl Reply, json_body: HashMap<String, String>) -> WebResult {
    info!("approve_post_handler:");
    let post_id = json_body.get("id").unwrap();
    let output = reject_post(post_id.clone()).await;
    match output {
        Ok(msg) => {
            let message: String = serde_json::to_string(&msg).unwrap();
            let response = GenericResponse::new(warp::http::StatusCode::OK, message.to_string());
            info!("response: {:?}", response);
            Ok(response)
        }
        Err(e) => {
            error!("approve_post_handler error: {:?}", e);
            let rejection = warp::reject::custom(InvalidPost);
            Err(rejection)
        }
    }
}