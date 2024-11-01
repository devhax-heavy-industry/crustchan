use super::handlers::*;
use crustchan::CONTENT_LIMIT;
use warp::filters::BoxedFilter;
use warp::{Filter, Reply};

pub fn post_route() -> BoxedFilter<(impl Reply,)> {
    warp::path!("api" / "posts")
    .and(warp::post())
        .and(warp::multipart::form())
        .and(warp::body::content_length_limit(CONTENT_LIMIT))
        .and(warp::addr::remote())
        .and_then(post_handler)
        .boxed()
}
pub fn posts_by_board_route() -> BoxedFilter<(impl Reply,)> {
    warp::path!("api" / "board" / String)
        .and(warp::get())
        .and_then(list_posts_by_board_handler)
        .boxed()
}
