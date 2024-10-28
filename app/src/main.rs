use crate::admin::admin_routes;
use std::env;
use std::net::Ipv4Addr;
// use boards::list_boards;
use crate::post::{post_routes,prog_posts_route};
use crate::board::board_routes;
use crate::rejections::handle_rejection;
use tracing_subscriber::fmt::format::FmtSpan;
use warp::{Filter, Rejection, Reply, reply::Json, http::Response};
use warp::http::header::{ HeaderValue, CONTENT_TYPE};
use warp::hyper::Body;
pub mod admin;
pub mod dynamodb;
pub mod model;
pub mod post;
pub mod board;
pub mod rejections;



#[derive(Debug)]
pub struct GenericResponse {
    pub status_code: warp::http::StatusCode,
    pub message: String,
}
impl Reply for GenericResponse {
         fn into_response(self) -> Response<Body> {
            let mut response = Response::new(self.message.into());
            response.headers_mut().insert(
                "Content-Type",
                HeaderValue::from_static("application/json"),
            );
            *response.status_mut() = self.status_code;
            response
         }
     }

    
impl GenericResponse {
    pub fn new(status_code: warp::http::StatusCode, message: String) -> Self {
        Self {
            status_code,
            message,
        }
    }
}
type WebResult<T> = std::result::Result<T, Rejection>;

const CONTENT_LIMIT: u64 = 1024 * 1024 * 25; // 25 MB

#[tokio::main]
async fn main() {
    let log_filter =
        std::env::var("RUST_LOG").unwrap_or_else(|_| "tracing=info,warp=debug,crustchan=debug".to_owned());
    tracing_subscriber::fmt()
        // .text()
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

    let routes = post_routes().boxed()
        .or(board_routes().boxed())
        .or(admin_routes().boxed())
        .with(warp::compression::gzip()) //; //.or(list_boards);
        .with(warp::log("crustchan"))
        .with(warp::trace::request())
        .recover(handle_rejection);

    warp::serve(routes).run((Ipv4Addr::LOCALHOST, port)).await
}


