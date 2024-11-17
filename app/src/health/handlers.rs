use tracing::info;
use crustchan::response::{GenericResponse, WebResult};

pub async fn health_handler() -> WebResult {
  info!("health_handler:");
  let response = GenericResponse::new(warp::http::StatusCode::OK, '{"status": "ok"}');
  Ok(response)
}