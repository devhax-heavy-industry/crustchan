pub mod admin;
pub mod auth;
pub mod board;
pub mod dynamodb;
pub mod middleware;
pub mod models;
pub mod post;
pub mod rejections;
pub mod response;

use std::env;
use std::net::Ipv4Addr;
// use boards::list_boards;
use crate::admin::admin_routes;
use crate::board::board_routes;
use crate::models::Admin;
use crate::post::post_routes;
use crate::rejections::handle_rejection;
use tracing::info;
use tracing_subscriber::fmt::format::FmtSpan;
use warp::{Filter, Rejection};



const CONTENT_LIMIT: u64 = 1024 * 1024 * 25; // 25 MB

pub async fn check_for_admin_user() -> Result<Admin, Rejection> {
    let admin_user = dynamodb::get_any_admin_user().await;
    match admin_user {
        Ok(admin) => {
            info!("An admin user exists");
            return Ok(admin);
        }
        Err(e) => {
            info!("No admin user exists, creating one now...{e:?}");

            let admin_user = Admin {
                username: "admin".to_string(),
                password: Admin::hash_password("changeme".to_string()).await.unwrap(),
                ..Default::default()
            };
            let _created_admin_output = dynamodb::create_admin(admin_user.clone()).await;
            let created_admin: Admin = dynamodb::get_admin_user(admin_user.username).await.unwrap();
            return Ok(created_admin);
        }
    }
}

#[tokio::main]
async fn main() {
    let log_filter = std::env::var("RUST_LOG")
        .unwrap_or_else(|_| "tracing=info,warp=debug,crustchan=debug".to_owned());
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

    // load up our project's routes
    let routes = post_routes()
        .boxed()
        .or(board_routes().boxed())
        .or(admin_routes().boxed())
        .with(warp::compression::gzip()) //; //.or(list_boards);
        .with(warp::log("crustchan"))
        .with(warp::trace::request())
        .recover(handle_rejection);

    // check for the existance of an admin user, creating one if not found
    let _admin = check_for_admin_user().await;

    // start the http server
    warp::serve(routes).run((Ipv4Addr::LOCALHOST, port)).await
}
