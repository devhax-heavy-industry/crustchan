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

#[derive(Debug)]
pub struct ConversionError;
impl reject::Reject for ConversionError {}

#[derive(Debug)]
pub struct HashError;
impl reject::Reject for HashError {}

#[derive(Debug)]
pub struct InvalidPost;
impl reject::Reject for InvalidPost {}

#[derive(Debug)]
pub struct InvalidDBConfig;
impl reject::Reject for InvalidDBConfig {}


pub enum Rejections {
    InvalidParameter(InvalidParameter),
    InvalidLogin(InvalidLogin),
    InvalidUser(InvalidUser),
    UnsupportedMediaType(UnsupportedMediaType),
    FileReadError(FileReadError),
    Unauthorized(Unauthorized),
    HashError(HashError),
    ConversionError(ConversionError),
    InvalidPost(InvalidPost)
}




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
    } else if let Some(_e) = err.find::<InvalidLogin>() {
        message = "Invalid Login";
        code = StatusCode::BAD_REQUEST;
    } else if let Some(_e) = err.find::<InvalidUser>() {
        message = "Invalid User";
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
