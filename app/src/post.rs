use crate::GenericResponse;
use crate::WebResult;
use bytes::BufMut;
use futures_util::TryStreamExt;
use warp::filters::BoxedFilter;
use warp::Reply;
use warp::multipart::FormData;
use warp::reply::json;
use warp::Filter;
use tracing::info;
use crate::CONTENT_LIMIT;
// POST /api/posts

pub fn post_route() -> BoxedFilter<(impl Reply,)> {
  warp::multipart::form()
    .and(warp::path("api"))
    .and(warp::path("posts"))
    .and(warp::body::content_length_limit(CONTENT_LIMIT))
    .and_then(post_handler)
    .boxed()
}


  pub async fn post_handler(form: FormData) -> WebResult<impl Reply> {
    info!("post_handler:");

    let field_names: Vec<_> = form
      .and_then(|mut field| async move {
        let mut bytes: Vec<u8> = Vec::new();
                info!("field name: {:?}", field.name());
                // field.data() only returns a piece of the content, you should call over it until it replies None
                while let Some(content) = field.data().await {
                    let content = content.unwrap();
                    bytes.put(content);
                }

                info!("field filename: {:?}", field.filename());
                Ok((
                  field.name().to_string(),
                  field.filename().unwrap().to_string(),
                  String::from_utf8_lossy(&*bytes).to_string(),
                ))
      })
      .try_collect()
      .await
      .unwrap();


    let message: String = format!("field_names {:?}", {field_names});

    let response_json = &GenericResponse {
        status: "success".to_string(),
        message,
    };
    info!("Response: {:?}", response_json);
    Ok(json(response_json))
}
