use warp::filters::BoxedFilter;
use warp::{Filter, Reply};

pub mod handlers;
pub mod routes;
pub use handlers::*;
pub use routes::*;
use utoipa::OpenApi;

#[derive(OpenApi)]
    #[openapi(paths(ban_handler, login_handler, approve_post_handler, reject_post_handler, create_board_handler))]
pub struct AdminApi;


pub fn admin_routes() -> BoxedFilter<(impl Reply,)> {
    admin_login_route()
    .or(admin_ban())
    .or(create_board_route())
    .or(admin_approve_post_route())
    .or(admin_reject_post_route())
    .or(admin_list_posts_by_session()).boxed()
}