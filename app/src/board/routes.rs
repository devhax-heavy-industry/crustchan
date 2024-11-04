use super::handlers::*;
use warp::filters::BoxedFilter;
use warp::{Filter, Reply};

pub fn get_board_by_id_route() -> BoxedFilter<(impl Reply,)> {
    // warp::get()
        warp::path!("api" / "board" / String / String)
        .and_then(get_post_by_id_handler)
        .boxed()
}

pub fn get_boards() -> BoxedFilter<(impl Reply,)> {
    // warp::get()
        warp::path!("api" / "board")
        .and_then(get_boards_handler)
        .boxed()
}
