use warp::filters::BoxedFilter;
use warp::{Reply, Filter};
use utoipa::OpenApi;

mod handlers;
mod routes;
pub use handlers::*;
pub use routes::*;

#[derive(OpenApi)]
    #[openapi(paths(list_posts_by_board_handler, post_handler))]
pub struct PostsApi;

pub fn post_routes() -> BoxedFilter<(impl Reply,)> {
    post_route().or(posts_by_board_route()).boxed()
}