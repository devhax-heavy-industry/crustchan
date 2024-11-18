use serde::Serialize;
use warp::http::header::HeaderValue;
use warp::hyper::Body;
use warp::reject::Rejection;
use warp::{http::Response, Reply};
use std::string::ToString;
pub type WebResult = std::result::Result<GenericResponse, Rejection>;

// pub trait New<'a, T> {
//     fn new<T:Serializer>(&mut self, status_code:&'a StatusCode, message: T) -> Self;
// }
// pub trait New<T> {
//     fn new<T:Serializer>(&mut self, status_code:StatusCode, message: T) -> Self;
// }

#[derive(Debug, Clone)]
pub struct GenericResponse {
    pub status_code: warp::http::StatusCode,
    pub message: String,
}
impl Default for GenericResponse {
    fn default() -> Self {
        Self {
            status_code: warp::http::StatusCode::OK,
            message: "".to_string(),
        }
    }
}
impl GenericResponse {
    pub fn new<E: Serialize>(status_code: warp::http::StatusCode, message: E) -> GenericResponse {
        GenericResponse { status_code: status_code, message: serde_json::to_string(&message).unwrap() }
    }
    pub fn new_from_string(status_code: warp::http::StatusCode, message: String) -> GenericResponse {
        GenericResponse { status_code: status_code, message: message.clone() }
    }
}

impl Reply for GenericResponse {
    fn into_response(self) -> Response<Body> {
        let bodystr = &self.message.clone();
        let body = Body::from(bodystr.to_string());
        let mut response = Response::new(body);
        response
            .headers_mut()
            .insert("Content-Type", HeaderValue::from_static("application/json"));
        *response.status_mut() = self.status_code;
        response
    }
}

#[derive(Debug, Clone)]
pub struct ApiError {
    pub status_code: warp::http::StatusCode,
    pub message: String,
}
impl Default for ApiError {
    fn default() -> Self {
        Self {
            status_code: warp::http::StatusCode::OK,
            message: "".to_string(),
        }
    }
}
impl ApiError {
    pub fn new(status_code: warp::http::StatusCode, message: String) -> ApiError {
        ApiError { status_code: status_code, message: message }
    }
}
impl Reply for ApiError {
    fn into_response(self) -> Response<Body> {
        let body = Body::from(serde_json::to_string(&self.message).unwrap());
        let mut response = Response::new(body);
        response
            .headers_mut()
            .insert("Content-Type", HeaderValue::from_static("application/json"));
        *response.status_mut() = self.status_code;
        response
    }
}
