use warp::filters::BoxedFilter;
use warp::{Filter, Reply};
use utoipa::OpenApi;

mod handlers;
pub mod routes;
pub use handlers::*;
pub use routes::*;

#[derive(OpenApi)]
    #[openapi(paths(get_boards_handler, get_post_by_id_handler))]
pub struct BoardsApi;

pub fn board_routes() -> BoxedFilter<(impl Reply,)> {
    get_board_by_id_route()
    .or(get_boards())
    .boxed()
}