use warp::filters::BoxedFilter;
use warp::{Filter, Reply};

mod handlers;
mod routes;
pub use handlers::*;
pub use routes::*;

pub fn post_routes_post() -> BoxedFilter<(impl Reply,)> {
    post_route()
}

pub fn post_routes_get() -> BoxedFilter<(impl Reply,)> {
    posts_by_board_route()
}