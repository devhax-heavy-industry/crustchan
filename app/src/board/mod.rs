use warp::filters::BoxedFilter;
use warp::{Filter, Reply};

mod handlers;
mod routes;
pub use handlers::*;
pub use routes::*;

pub fn board_routes() -> BoxedFilter<(impl Reply,)> {
    get_board_by_id_route().or(get_boards()).boxed()
}
