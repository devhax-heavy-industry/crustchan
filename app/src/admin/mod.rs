use warp::filters::BoxedFilter;
use warp::{Filter, Reply};

mod handlers;
mod routes;
pub use handlers::*;
pub use routes::*;

pub fn admin_routes() -> BoxedFilter<(impl Reply,)> {
    admin_login_route()
    .or(admin_ban())
    .or(create_board_route())
    .or(admin_approve_post_route())
    .or(admin_list_posts_by_session()).boxed()
}


pub fn admin_routes_get() -> BoxedFilter<(impl Reply,)> {
    admin_list_posts_by_session()
}
pub fn admin_routes_post() -> BoxedFilter<(impl Reply,)> {
    admin_login_route()
    .or(admin_ban())
    .or(create_board_route())
    .or(admin_approve_post_route()).boxed()
}
