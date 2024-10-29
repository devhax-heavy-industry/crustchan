use serde::{Deserialize, Deserializer, Serializer, Serialize};
use warp::http::header::HeaderValue;
use warp::hyper::Body;
use warp::reject::Rejection;
use warp::{http::Response, Reply};

pub type WebResult<T> = std::result::Result< GenericResponse<T>, Rejection>;

// pub trait New<'a, T> {
//     fn new<T:Serializer>(&mut self, status_code:&'a StatusCode, message: T) -> Self;
// }
// pub trait New<T> {
//     fn new<T:Serializer>(&mut self, status_code:StatusCode, message: T) -> Self;
// }
∂
#[derive(Debug, Clone)]
pub struct GenericResponse<T> {
    pub status_code: warp::http::StatusCode,
    pub message: T,
}
impl<T> Default for GenericResponse<T> 
    where T: Default {
    fn default() -> Self {
        Self {
            status_code: warp::http::StatusCode::OK,
            message: T::default(),
        }
    }
}
impl<'a,T> GenericResponse<T> {
    pub fn new<E:Serialize + Default +Deserialize<'a>>(status_code: warp::http::StatusCode, message: E) -> GenericResponse<E> {
                let mut ret = GenericResponse::default();
                ret.status_code = status_code;
                ret.message = message;
                ret
            }
}

impl<T> Reply for GenericResponse<T> 
where T: Serialize + std::marker::Send {
        fn into_response(self) -> Response<Body> {
            let mut response = Response::new(serde_json::to_string(&self.message).unwrap().into());
            response
                .headers_mut()
                .insert("Content-Type", HeaderValue::from_static("application/json"));
            *response.status_mut() = self.status_code;
            response
        }
    }

#[derive(Debug, Clone)]
pub struct ApiError<T> {
    pub status_code: warp::http::StatusCode,
    pub message: T,
}
impl<T> Default for ApiError<T>
where T: Default {
    fn default() -> Self {
        Self {
            status_code: warp::http::StatusCode::OK,
            message: T::default(),
        }
    }
}
impl<'a,T>  ApiError<T> {
    pub fn new<E:Serialize + Default +Deserialize<'a>>(status_code: warp::http::StatusCode, message: E) -> ApiError<E> {
        let mut ret = ApiError::default();
        ret.status_code = status_code;
        ret.message = message;
        ret
    }
}
impl<T> Reply for ApiError<T> 
where T: Serialize + std::marker::Send {
    fn into_response(self) -> Response<Body> {
        let mut response = Response::new(serde_json::to_string(&self.message).unwrap().into());
        response
            .headers_mut()
            .insert("Content-Type", HeaderValue::from_static("application/json"));
        *response.status_mut() = self.status_code;
        response
    }
}