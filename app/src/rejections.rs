use std::convert::Infallible;
use crate::GenericResponse;
use tracing::info;
use warp::http::StatusCode;
use warp::{reject, Rejection, Reply};

#[derive(Debug)]
pub struct InvalidParameter;

impl reject::Reject for InvalidParameter {}

#[derive(Debug)]
pub struct UnsupportedMediaType;

impl reject::Reject for UnsupportedMediaType {}

#[derive(Debug)]
pub struct FileReadError;

impl reject::Reject for FileReadError {}

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
        code = StatusCode::METHOD_NOT_ALLOWED;
        message = "METHOD_NOT_ALLOWED";
    } else {
        // We should have expected this... Just log and say its a 500
        eprintln!("unhandled rejection: {:?}", err);
        code = StatusCode::INTERNAL_SERVER_ERROR;
        message = "UNHANDLED_REJECTION";
    }


    let response = GenericResponse::new(code, message.into());

    Ok(warp::reply::with_status(response, code))
}