use super::handlers::*;
use warp::filters::BoxedFilter;
use warp::{Filter, Reply};
use crustchan::models::{Board, Post};

#[utoipa::path(
        get,
        path = "/api/board/{board_id}/{post_id}",
        responses(
            (status = 200, description = "Post found successfully", body = Post),
        ),
        params(
            ("board_id" = String, Path, description="The board id"),
            ("post_id" = String, Path, description = "The post id"),
        ),
    )]
pub fn get_board_by_id_route() -> BoxedFilter<(impl Reply,)> {
        warp::path!("api" / "board" / String / String)
        .and(warp::get())
        .and_then(get_post_by_id_handler)
        .boxed()
}

#[utoipa::path(get,
        path = "/api/boards",
        responses(
            (status = 200, description = "Boards found successfully", body = Vec<Board>),
        ),
    )]
pub fn get_boards() -> BoxedFilter<(impl Reply,)> {
        warp::path!("api" / "board")
        .and(warp::get())
        .and_then(get_boards_handler)
        .boxed()
}

// todo, return top level posts with most recent replies
// pub fn get_board_posts() -> BoxedFilter<(impl Reply,)> {
//         warp::path!("api" / "board" / String / "posts")
//         .and(warp::get())
//         .and_then(get_boards_handler)
//         .boxed()
// }