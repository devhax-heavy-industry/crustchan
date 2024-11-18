use warp::filters::BoxedFilter;
use warp::{Reply};

pub mod handlers;
pub mod routes;
pub use handlers::*;
pub use routes::*;

pub fn openapi_routes() -> BoxedFilter<(impl Reply,)> {
  openapi_route()
}