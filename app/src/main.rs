use std::env;
use serde::Serialize;
use std::net::Ipv4Addr;
use crate::admin::admin_routes;
// use boards::list_boards;
use crate::post::post_route;
use warp::{Filter, Rejection};
use tracing_subscriber::fmt::format::FmtSpan;

pub mod admin;
pub mod post;

#[derive(Serialize)]
#[derive(Debug)]
pub struct GenericResponse {
    pub status: String,
    pub message: String,
}

type WebResult<T> = std::result::Result<T, Rejection>;

const CONTENT_LIMIT: u64 = 1024 * 1024 * 25; // 25 MB

#[tokio::main]
async fn main() {
  let log_filter = std::env::var("RUST_LOG").unwrap_or_else(|_| "tracing=info,warp=debug".to_owned());
  tracing_subscriber::fmt()
  .json()
  .with_thread_names(false)
  .with_max_level(tracing::Level::DEBUG)
  // Use the filter we built above to determine which traces to record.
  .with_env_filter(log_filter)
  // Record an event when each span closes. This can be used to time our
  // routes' durations!
  .with_span_events(FmtSpan::CLOSE)
  .init();
    let port_key = "FUNCTIONS_CUSTOMHANDLER_PORT";
    let port: u16 = match env::var(port_key) {
        Ok(val) => val.parse().expect("Custom Handler port is not a number!"),
        Err(_) => 3000,
    };


    let routes = post_route()
      .or(admin_routes())
      .with(warp::compression::gzip())//; //.or(list_boards);
      .with(warp::trace::request());

    warp::serve(routes).run((Ipv4Addr::LOCALHOST, port)).await
}