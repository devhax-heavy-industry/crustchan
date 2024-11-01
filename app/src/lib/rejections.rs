use crate::response::ApiError;
use std::convert::Infallible;
use tracing::info;
use warp::http::StatusCode;
use warp::{reject, Rejection, Reply};

#[derive(Debug)]
pub struct InvalidParameter;

impl reject::Reject for InvalidParameter {}

#[derive(Debug)]
pub struct InvalidLogin;

impl reject::Reject for InvalidLogin {}

#[derive(Debug)]
pub struct InvalidUser;

impl reject::Reject for InvalidUser {}

#[derive(Debug)]
pub struct UnsupportedMediaType;

impl reject::Reject for UnsupportedMediaType {}

#[derive(Debug)]
pub struct FileReadError;

impl reject::Reject for FileReadError {}
#[derive(Debug)]
pub struct Unauthorized;

impl reject::Reject for Unauthorized {}

pub enum Rejections {
    InvalidParameter(InvalidParameter),
    InvalidLogin(InvalidLogin),
    InvalidUser(InvalidUser),
    UnsupportedMediaType(UnsupportedMediaType),
    FileReadError(FileReadError),
    Unauthorized(Unauthorized),
    HashError(HashError),
}
impl reject::Reject for HashError {}

#[derive(Debug)]
pub struct HashError;

// impl From<argonautica::Error> for Rejections {
//     fn from(_e: argonautica::Error) -> Self {
//         Rejections(HashError)
//     }
// }
// impl From<ApiError<T>> for Rejection {
//     fn from(e: ApiError<T>) -> Self {
//         warp::reject::custom(e.message)
//     }
// }
// impl From<argonautica::Error> for HashError {
//     fn from(e: argonautica::Error) -> Self {
//         match e.kind() {
//             _ => {
//                 dbg!(e.kind());
//                 Rejections::Unauthorized
//             }
//         }
//     }
// }

pub async fn handle_rejection(err: Rejection) -> Result<impl Reply, Infallible> {
    let code;
    let message;

    info!("Handling a rejection {:?}", err);
    if err.is_not_found() {
        code = StatusCode::NOT_FOUND;
        message = "NOT_FOUND";
    } else if let Some(_e) = err.find::<InvalidParameter>() {
        message = "BAD_REQUEST";
        code = StatusCode::BAD_REQUEST;
    } else if let Some(_e) = err.find::<UnsupportedMediaType>() {
        message = "UNSUPPORTED_MEDIA_TYPE";
        code = StatusCode::UNSUPPORTED_MEDIA_TYPE;
    } else if let Some(_e) = err.find::<FileReadError>() {
        message = "FILE_READ_ERROR";
        code = StatusCode::BAD_REQUEST;
    } else if let Some(_) = err.find::<warp::reject::MethodNotAllowed>() {
        // We can handle a specific error, here METHOD_NOT_ALLOWED,
        // and render it however we want
        info!("MethodNotAllowed {:?}", err);
        code = StatusCode::METHOD_NOT_ALLOWED;
        message = "METHOD_NOT_ALLOWED";
    } else {
        // We should have expected this... Just log and say its a 500
        eprintln!("unhandled rejection: {:?}", err);
        code = StatusCode::INTERNAL_SERVER_ERROR;
        message = "UNHANDLED_REJECTION";
    }

    let response: ApiError = ApiError::new(code, message.to_string());

    Ok(warp::reply::with_status(response, code))
}
