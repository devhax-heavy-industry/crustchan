use tracing::info;
use crustchan::response::{GenericResponse, WebResult};

#[utoipa::path(
  get,
  path = "/health",
  responses(
          (status = 200, description = "Pet found successfully", body = ()),
  ),
)]
pub async fn health_handler() -> WebResult {
  info!("health_handler:");
  let response = GenericResponse::new_from_string(warp::http::StatusCode::OK, "{\"status\": \"ok\"}".to_string());
  Ok(response)
}