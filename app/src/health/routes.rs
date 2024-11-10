use super::handlers::*;
use warp::filters::BoxedFilter;
use warp::{Filter, Reply};

pub fn health_route() -> BoxedFilter<(impl Reply,)> {
        warp::path!("health")
        .and(warp::get())
        .and_then(health_handler).boxed()
}