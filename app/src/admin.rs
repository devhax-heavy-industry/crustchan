use crate::GenericResponse;
use crate::WebResult;
use warp::filters::BoxedFilter;
use warp::Reply;
use warp::Filter;
use warp::reply::json;
use tracing::info;
use crate::CONTENT_LIMIT;


pub fn admin_ban() -> BoxedFilter<(impl Reply,)> {
  warp::post()
    .and(warp::path("api"))
    .and(warp::path("admin"))
    .and(warp::path("ban"))
    .and(warp::body::content_length_limit(CONTENT_LIMIT))
    .and_then(ban_handler)
    .boxed()
}
pub fn admin_list_posts_by_session() -> BoxedFilter<(impl Reply,)> {
  warp::get().and(warp::path("api")).and(warp::path("admin")).and(warp::path("posts-by-session")).and_then(ban_handler).boxed()
}

pub fn admin_list_posts_by_ip() -> BoxedFilter<(impl Reply,)> {
  warp::get().and(warp::path("api")).and(warp::path("admin")).and(warp::path("posts-by-ip")).and_then(ban_handler).boxed()
}

pub fn admin_routes() -> BoxedFilter<(impl Reply,)> {
   admin_ban().or(admin_list_posts_by_session()).or(admin_list_posts_by_ip()).boxed()
}

pub async fn ban_handler() -> WebResult<impl Reply> {
  info!("ban_handler:");
  const MESSAGE: &str = "Build Simple CRUD API with Rust";

  let response_json = &GenericResponse {
      status: "success".to_string(),
      message: MESSAGE.to_string(),
  };
  info!("Response: {:?}", response_json);
  Ok(json(response_json))
}
