use warp::filters::BoxedFilter;
use warp::{Filter, Reply};

mod handlers;
mod routes;
pub use handlers::*;
pub use routes::*;

pub fn admin_routes() -> BoxedFilter<(impl Reply,)> {
    admin_ban()
        .boxed()
        .or(admin_list_posts_by_session().boxed())
        .or(admin_list_posts_by_ip().boxed())
        .or(create_board_route().boxed())
        .boxed()
}
