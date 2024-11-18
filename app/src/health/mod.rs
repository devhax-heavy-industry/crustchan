use warp::filters::BoxedFilter;
use warp::Reply;
use utoipa::OpenApi;

mod handlers;
mod routes;
pub use handlers::*;
pub use routes::*;

#[derive(OpenApi)]
    #[openapi(paths(health_handler))]
pub struct HealthApi;


pub fn health_routes() -> BoxedFilter<(impl Reply,)> {
  health_route()
}