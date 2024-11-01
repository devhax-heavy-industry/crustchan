use anyhow::Result;
use crustchan::dynamodb::{create_post, list_posts_by_board};
use crustchan::models::Post;
use crustchan::response::{GenericResponse, WebResult};
use crustchan::AWS_REGION;
use std::path::PathBuf;
// use futures_util::{FutureExt, StreamExt, TryStreamExt};
use image::ImageReader;
use std::net::SocketAddr;
use std::path::Path;
use tracing::{error, info,warn};
use warp::multipart::FormData;
use std::io::Read;
use rusoto_s3::S3;
use futures::future;
use futures::TryStreamExt;
use futures_util::StreamExt;
use warp::Buf;

pub async fn post_handler(mut form: FormData, addr: Option<SocketAddr>) -> WebResult {
    info!("post_handler:");
    let mut post: Post = Post::default();
    while let Ok(Some(part)) = form.try_next().await {
        let name: String = part.name().to_string();
        let filename: String = match part.filename() {
            Some(filename) => filename.to_string(),
            None => "".to_string(), //return Ok(GenericResponse::new(warp::http::StatusCode::BAD_REQUEST, "Missing filename".to_string())),
        };
        if filename == "" {
            let values_res = part
                .stream()
                .then(|result| {
                    let binding = result.unwrap();
                    let slice = binding.chunk();
                    let value_as_str = std::str::from_utf8(slice).unwrap();
                    future::ok::<_, ()>(value_as_str.to_string())
                })
                .collect::<Vec<_>>()
                .await;
            let value_str: String = values_res.into_iter().map(|x| x.unwrap()).collect();

            match name.as_str() {
                "subject" => post.subject = value_str.to_string(),
                "text" => post.text = value_str.to_string(),
                "board_id" => post.board_id = value_str.to_string(),
                "name" => post.poster = value_str.to_string(),
                "op" => post.op = value_str.to_string(),
                _ => info!("name not found in form input {name}"), //Err(warp::reject::custom(InvalidParameter))?,
            }
        } else {
            println!("Receiving file: {}", filename);

            let mut data = Vec::new();
            let mut stream = part.stream();

            while let Ok(Some(mut chunk)) = stream.try_next().await {
                let slice: &[u8] = chunk.chunk();
                data.extend_from_slice(slice);
                chunk.advance(slice.len());
            }
            let uuidv4 = uuid::Uuid::new_v4();
            let save_path = PathBuf::from(format!("/tmp/uploads/{}", filename));
            let extension = save_path.extension().unwrap().to_str().unwrap();
            let new_filename = format!("{}.{}", uuidv4, extension);
            let filepath = save_path.to_str().unwrap().to_string();
            post.file =new_filename.to_owned();
            post.file_name = new_filename.to_owned();
            post.file_size = data.len() as u64;
            post.file_original_name = filename.to_owned();
            let created_dir_result = tokio::fs::create_dir_all(PathBuf::from("/tmp/uploads")).await;
            match created_dir_result {
                Err(_) => {
                    return Ok(GenericResponse::new(
                        warp::http::StatusCode::INTERNAL_SERVER_ERROR,
                        "Failed to create files dir".to_string(),
                    ))
                }
                Ok(_) => {
                    info!("Dir created successfully");
                }
            }
            match tokio::fs::write(&save_path, &data).await {
                Err(_) => {
                    return Ok(GenericResponse::new(
                        warp::http::StatusCode::INTERNAL_SERVER_ERROR,
                        "Failed to save file".to_string(),
                    ))
                }
                Ok(_) => {
                    info!("File saved successfully");
                }
            };



            let file_dimensions = get_image_dimensions(filepath.as_str()).unwrap();
            post.file_dimensions = format!("{}x{}", file_dimensions.0, file_dimensions.1);

            let s3_response = upload_to_s3(&save_path, post.clone().file_name).await;
            match s3_response {
                Ok(_) => {
                    info!("File uploaded to S3 successfully");
                }
                Err(_) =>{ warn!("Failed to upload file to S3");}
            }
        }
    }
    post.ip = addr.unwrap().to_string();
    dbg!(&post.clone());
    // Ok(GenericResponse::new(warp::http::StatusCode::CREATED, "File uploaded".to_string()));

    if post.board_id.is_empty() {
        let response = GenericResponse::new(
            warp::http::StatusCode::BAD_REQUEST,
            "Board must be supplied".to_string(),
        );

        return Ok(response);
    }
    if post.text.is_empty() {
        error!("Text body must be supplied");
        return Err(warp::reject::reject());
    }
    if post.poster.is_empty() {
        post.poster = "Anonymous".to_string();
    }
    if post.op.is_empty() {
        post.op = "NULL".to_string();
    }

    // create db entry
    let __db_post = create_post(post.clone()).await.unwrap();
    let response = GenericResponse::new(warp::http::StatusCode::CREATED, post.clone());
    info!("Response: {:?}", response);
    Ok(response)
}

pub async fn upload_to_s3(path:&Path, new_filename:String) -> Result<rusoto_s3::PutObjectOutput, Box<dyn std::error::Error>> {
    let bucket = std::env::var("S3_BUCKET").unwrap();
    let client = rusoto_s3::S3Client::new(AWS_REGION);

    let file = std::fs::File::open(path).unwrap();
    let key = format!("uploads/{}", new_filename);
    let mut buffer = Vec::new();
    file.take(1024 * 1024 * 25).read_to_end(&mut buffer).unwrap();

    let request = rusoto_s3::PutObjectRequest {
        bucket: bucket,
        key: key,
        body: Some(buffer.into()),
        ..Default::default()
    };

    let response = client.put_object(request).await?;
    Ok(response)
}

pub async fn list_posts_by_board_handler(board_id: String) -> WebResult {
    info!("list_posts_by_board_handler:");
    let posts = list_posts_by_board(board_id).await.unwrap();

    let response = GenericResponse::new(warp::http::StatusCode::OK, posts);
    Ok(response)
}

fn get_image_dimensions(file_path: &str) -> Result<(u32, u32)> {
    let path = Path::new(file_path);
    let reader = ImageReader::open(path)?;
    let dimensions = reader.into_dimensions()?;
    Ok(dimensions)
}
