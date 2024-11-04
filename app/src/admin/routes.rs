use super::handlers::*;
use crustchan::CONTENT_LIMIT;
use warp::filters::BoxedFilter;
use warp::{Filter, Reply};

pub fn admin_ban() -> BoxedFilter<(impl Reply,)> {
    // warp::post()
    warp::path!("api" / "admin" / "ban")
        .and(crate::middleware::authn())
        .and(warp::body::content_length_limit(CONTENT_LIMIT))
        .and_then(ban_handler)
        .boxed()
}
pub fn admin_list_posts_by_session() -> BoxedFilter<(impl Reply,)> {
    // warp::get()
    warp::path!("api" / "admin" / "posts-by-session")
        .and(crate::middleware::authn())
        .and_then(ban_handler)
        .boxed()
}

pub fn admin_list_posts_by_ip() -> BoxedFilter<(impl Reply,)> {
    // warp::get()
    warp::path!("api" / "admin" / "posts-by-ip")
        .and(crate::middleware::authn())
        .and_then(admin_lists_posts_by_ip_handler)
        .boxed()
}

pub fn create_board_route() -> BoxedFilter<(impl Reply,)> {
    // warp::post()
    warp::path!("api" / "admin" / "board")
        .and(crate::middleware::cookie_authn())
        .and(warp::body::content_length_limit(CONTENT_LIMIT))
        .and(warp::body::json())
        .and_then(create_board_handler)
        .boxed()
}
pub fn admin_login_route() -> BoxedFilter<(impl Reply,)> {
    // warp::post()
    warp::path!("api" / "admin" / "login")
        .and(warp::body::content_length_limit(CONTENT_LIMIT))
        .and(warp::body::json())
        .and_then(login_handler)
        .boxed()
}

pub fn admin_approve_post_route() -> BoxedFilter<(impl Reply,)> {
    // warp::post()
    warp::path!("api" / "admin" / "posts" / "approve")
        .and(crate::middleware::cookie_authn())
        .and(warp::body::content_length_limit(CONTENT_LIMIT))
        .and(warp::body::json())
        .and_then(approve_post_handler)
        .boxed()
}
