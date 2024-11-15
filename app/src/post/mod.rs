use warp::filters::BoxedFilter;
use warp::{Reply, Filter};

mod handlers;
mod routes;
pub use handlers::*;
pub use routes::*;

pub fn post_routes() -> BoxedFilter<(impl Reply,)> {
    post_route().or(posts_by_board_route()).boxed()
}