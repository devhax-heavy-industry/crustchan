use super::handlers::*;
use warp::filters::BoxedFilter;
use warp::{Filter, Reply};

pub fn openapi_route() -> BoxedFilter<(impl Reply,)> {
        warp::path!("api-docs"/"openapi.json")
        .and(warp::get())
        .and_then(openapi_handler).boxed()
}