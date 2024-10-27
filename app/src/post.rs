use crate::dynamodb::{create_post, list_posts_by_board};
use crate::model::Post;
use crate::rejections::{FileReadError, InvalidParameter, UnsupportedMediaType};
use crate::GenericResponse;
use crate::WebResult;
use crate::CONTENT_LIMIT;
use anyhow::Result;
use bytes::BufMut;
use futures_util::TryStreamExt;
use image::ImageReader;
use std::ffi::OsStr;
use std::fs;
use std::net::SocketAddr;
use std::path::Path;
use tracing::{error, info};
use uuid::Uuid;
use warp::filters::BoxedFilter;
use warp::multipart::FormData;
use warp::{Buf, Filter, Reply, Rejection};
// POST /api/posts

pub fn post_route() -> BoxedFilter<(impl Reply,)> {
    warp::multipart::form()
        .and(warp::path("api"))
        .and(warp::path("posts"))
        .and(warp::body::content_length_limit(CONTENT_LIMIT))
        .and(warp::addr::remote())
        .and_then(post_handler)
        .boxed()
}

pub async fn post_handler(form: FormData, addr: Option<SocketAddr>) -> WebResult<impl Reply> {
    info!("post_handler:");
    let mut parts = form.into_stream();
    let mut post: Post = Post::default();
    while let Ok(Some(p)) = parts.try_next().await {
        if p.filename() != None {
            {
                let filename = p.filename().unwrap().to_owned();
                let filepath = format!(
                    "/tmp/uploads/{}.{}",
                    Uuid::new_v4(),
                    get_extension_from_filename(filename.as_str()).unwrap()
                );

                // validate mime types
                let mime = p.content_type().to_owned().unwrap();
                if !mime.starts_with("image") {
                    return Err(warp::reject::custom(UnsupportedMediaType));
                }

                // copy to /tmp dir
                fs::create_dir_all("/tmp/uploads").unwrap();
                save_part_to_file(&filepath, p).await.expect("save error");
                let dimensions = get_image_dimensions(filepath.as_str()).unwrap();
                post.file = filepath.to_owned();
                post.file_name = filename.to_owned();
                post.file_size = fs::metadata(&filepath).unwrap().len();
                post.file_original_name = filename.to_owned();
                post.file_dimensions = format!("{}x{}", dimensions.0, dimensions.1);
            }
        } else {
            let field_name = p.name().to_owned();
            {
                let field_value = p
                    .stream()
                    .try_fold(Vec::new(), |mut vec, data| {
                        vec.put(data);
                        async move { Ok(vec) }
                    })
                    .await
                    .map_err(|e| {
                        error!("reading file error: {}", e);
                        let _ = Err::<GenericResponse, Rejection>(warp::reject::custom(FileReadError));
                    });
                let value_string = String::from_utf8(field_value.unwrap()).unwrap().to_owned();
                post.ip = addr.unwrap().to_string();
                match field_name.as_str() {
                    "subject" => post.subject = value_string.to_string(),
                    "text" => post.text = value_string.to_string(),
                    "board_id" => post.board_id = value_string.to_string(),
                    "poster" => post.poster = value_string.to_string(),
                    "op" => post.op = value_string.to_string(),
                    _ => Err(warp::reject::custom(InvalidParameter))?,
                }
                info!("field {:?}: {:?}", field_name, value_string.to_owned());
            }
        }
    }

    if post.board_id.is_empty() {
      let response = GenericResponse::new(
          warp::http::StatusCode::BAD_REQUEST,
          "Board must be supplied".to_string());

        return Ok(response);
    }
    if post.text.is_empty() {
        error!("Text body must be supplied");
        return Err(warp::reject::reject());
    }
    if post.poster.is_empty() {
        post.poster = "Anonymous".to_string();
    }

    // create db entry
    let __db_post = create_post(post.clone()).await.unwrap();
    let message: String = format!("post: {:?}", post.clone());

    let response= GenericResponse::new(warp::http::StatusCode::CREATED, message); 
    info!("Response: {:?}", response);
    Ok(response)
}

pub fn posts_by_board_route() -> BoxedFilter<(impl Reply,)> {
    warp::get()
      .and(warp::path!("api" / "board" / String))
      .and_then(list_posts_by_board_handler)
      .boxed()
}
pub async fn list_posts_by_board_handler( board_id:String) -> WebResult<impl Reply> {
    info!("list_posts_by_board_handler:");
    let posts = list_posts_by_board(board_id).await.unwrap();


    let message: String = format!("{:?}", posts);

    let response = GenericResponse::new(warp::http::StatusCode::OK, message);
    Ok(response)
}



async fn save_part_to_file(path: &str, part: warp::multipart::Part) -> Result<(), std::io::Error> {
    let data = part
        .stream()
        .try_fold(Vec::new(), |mut acc, buf| async move {
            acc.extend_from_slice(buf.chunk());
            Ok(acc)
        })
        .await
        .expect("folding error");
    std::fs::write(path, data)
}

fn get_extension_from_filename(filename: &str) -> Option<&str> {
    Path::new(filename).extension().and_then(OsStr::to_str)
}
fn get_image_dimensions(file_path: &str) -> Result<(u32, u32)> {
    let path = Path::new(file_path);
    let reader = ImageReader::open(path)?;
    let dimensions = reader.into_dimensions()?;
    Ok(dimensions)
}
