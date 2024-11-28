use serde_json::to_string;
use crustchan::response::{GenericResponse, WebResult};
use tracing::info;

use utoipa::OpenApi;

use crate::board::BoardsApi;
use crate::health::HealthApi;
use crate::post::PostsApi;
use crate::admin::AdminApi;

// let openapi = OpenApi::new(Info::new("pet api", "0.1.0"), Paths::new());

#[derive(OpenApi)]
#[openapi( 
  nest(
    (path = "/api/boards", api = BoardsApi),
    (path = "/", api = HealthApi),
    (path = "/", api = PostsApi),
    (path = "/", api = AdminApi)
  ),
)]
struct ApiDoc;

#[utoipa::path(
  get,
  path = "/api-docs/openapi.json",
  responses(
    (status = 200, description = "JSON file", body = ())))]
pub async fn openapi_handler() -> WebResult {
  info!("openapi_handler:");
  let json = to_string(&ApiDoc::openapi());
  let response = GenericResponse::new_from_string(warp::http::StatusCode::OK, json.unwrap());
  Ok(response)
}
