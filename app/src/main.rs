pub mod admin;
pub mod auth;
pub mod board;
pub mod middleware;
pub mod post;

use std::env;
use std::net::Ipv4Addr;
// use boards::list_boards;
use admin::{admin_routes_get, admin_routes_post};
use auth::hash_password;
use board::board_routes_get;
use crustchan::dynamodb;
use crustchan::models::Admin;
use crustchan::rejections::handle_rejection;
use post::{post_routes_get, post_routes_post};
use tracing::info;
use tracing_subscriber::fmt::format::FmtSpan;
use warp::{Filter, Rejection};


pub async fn check_for_admin_user() -> Result<Admin, Rejection> {
    let admin_user = dynamodb::get_any_admin_user().await;
    match admin_user {
        Ok(admin) => {
            info!("An admin user exists");
            return Ok(admin);
        }
        Err(e) => {
            info!("No admin user exists, creating one now...{e:?}");
            let password =hash_password("changeme".to_string()).unwrap();
            dbg!(&password);
            let admin_user = Admin {
                username: "admin".to_string(),
                password,
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
    let static_route = warp::fs::dir("static");
    let get_routes =
        warp::get()
        .and(static_route
        .or(admin_routes_get())
        .or(board_routes_get())
        .or(post_routes_get()));
    let post_routes =warp::post()
        .and(admin_routes_post().or(post_routes_post()));
    let log_filter = std::env::var("RUST_LOG")
        .unwrap_or_else(|_| "tracing=info,warp=debug,crustchan=trace,crustchan-api=trace".to_owned());
    tracing_subscriber::fmt()
        // .text()
        .with_thread_names(false)
        .with_max_level(tracing::Level::DEBUG)
        // Use the filter we built above to determine which traces to record.
        .with_env_filter(log_filter)
        // Record an event when each span closes. This can be used to time our
        // routes' durations!
        .with_span_events(FmtSpan::NEW)
        .init();
    let port_key = "FUNCTIONS_CUSTOMHANDLER_PORT";
    let port: u16 = match env::var(port_key) {
        Ok(val) => val.parse().expect("Custom Handler port is not a number!"),
        Err(_) => 3000,
    };

    // load up our project's routes
    let routes =
        post_routes 
        .or(get_routes)
        .with(warp::compression::gzip())
        .with(warp::log("crustchan-api"))
        .with(warp::trace::request())
        .recover(handle_rejection);

    // check for the existence of an admin user, creating one if not found
    let _admin = check_for_admin_user().await;

    // start the http server
    warp::serve(routes).run((Ipv4Addr::LOCALHOST, port)).await
}
