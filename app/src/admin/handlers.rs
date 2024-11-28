use crate::auth::{AuthnToken,login};
use crustchan::dynamodb::{approve_post, create_board, reject_post};
use crustchan::models::{Board, FetchPostInput};
use crustchan::rejections::{InvalidUser, InvalidPost};
use crustchan::response::{GenericResponse, WebResult};
use utoipa::ToSchema;
use std::collections::HashMap;
use tracing::{error, info};
use warp::reply::WithHeader;
use warp::Rejection;
use warp::Reply;

#[utoipa::path(
    get,
    path = "api/admin/ban",
    responses(
            (status = 200, description = "User successfully banned", body = ()),
    ),
  )]
pub async fn ban_handler(_token: impl Reply) -> WebResult {
    info!("ban_handler:");
    const MESSAGE: &str = "lel ban em all";

    let response = GenericResponse::new(warp::http::StatusCode::OK, MESSAGE.to_string());
    info!("response: {:?}", response);
    Ok(response)
}

#[utoipa::path(
    post,
    path = "api/admin/board",
    responses(
            (status = 200, description = "Board created successfully", body = Board),
    ),
  )]
pub async fn create_board_handler(
    _token: AuthnToken,
    json_body: HashMap<String, String>,
) -> WebResult {
    info!("create_board_handler:");
    let json_str = serde_json::to_string(&json_body).unwrap();
    let board: Board = serde_json::from_str(&json_str).unwrap();
    let __db_board: rusoto_dynamodb::PutItemOutput = create_board(board.clone()).await.unwrap();
    // let message: String = format!("board: {:?}", board.clone());

    let response = GenericResponse::new(warp::http::StatusCode::OK, board.clone());
    Ok(response)
}


#[derive(Debug, Clone, ToSchema)]
pub struct PostByIpInput {
    ip: String
}

#[utoipa::path(
    get,
    path = "api/admin/posts_by_ip",
    request_body = PostByIpInput,
    responses(
            (status = 200, description = "Posts found successfully", body = Vec<String>),
    ),
  )]
pub async fn admin_lists_posts_by_ip_handler(_token: impl Reply) -> WebResult {
    info!("admin_lists_posts_by_ip_handler:");
    let posts = vec!["post1", "post2", "post3"];
    let response = GenericResponse::new(warp::http::StatusCode::OK, posts);
    Ok(response)
}

#[utoipa::path(
    post,
    path = "api/admin/login",
    responses(
        (status = 200, description = "User logged in successfully", body = ()),
        (status = 400, description = "Invalid User/pass", body = ()),
    ),
  )]
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
                GenericResponse::new_from_string(warp::http::StatusCode::OK, msg),
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



#[utoipa::path(
    post,
    path = "api/admin/posts/approve",
    request_body = FetchPostInput, 
    responses(
            (status = 200, description = "Post approved successfully", body = ()),
    ),
  )]
pub async fn approve_post_handler(_token: impl Reply, json_body: HashMap<String, String>) -> WebResult {
    info!("approve_post_handler:");
    let post_id = json_body.get("id").unwrap();

    let output = approve_post(post_id.clone()).await;
    match output {
        Ok(msg) => {
            let response = GenericResponse::new(warp::http::StatusCode::OK, msg);
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

#[utoipa::path(
    post,
    path = "api/admin/posts/reject",
    request_body = FetchPostInput, 
    responses(
            (status = 200, description = "Post rejected successfully", body = ()),
    ),
  )]
pub async fn reject_post_handler(_token: impl Reply, json_body: HashMap<String, String>) -> WebResult {
    info!("approve_post_handler:");
    let post_id = json_body.get("id").unwrap();
    let output = reject_post(post_id.clone()).await;
    match output {
        Ok(msg) => {
            let response = GenericResponse::new(warp::http::StatusCode::OK, msg);
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