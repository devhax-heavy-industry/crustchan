use warp::filters::BoxedFilter;
use warp::{Filter, Reply};

mod handlers;
mod routes;
pub use handlers::*;
pub use routes::*;

pub fn health_routes() -> BoxedFilter<(impl Reply,)> {
  health_route()
}