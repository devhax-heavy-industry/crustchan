use crustchan::dynamodb::{create_post, list_posts_by_board};
use crustchan::models::Post;
use crustchan::response::{WebResult, GenericResponse};
use anyhow::Result;
use std::path::PathBuf;
// use futures_util::{FutureExt, StreamExt, TryStreamExt};
use image::ImageReader;
use std::ffi::OsStr;
use std::net::SocketAddr;
use std::path::Path;
use tracing::{error, info};
use warp::multipart::FormData;

use futures::TryStreamExt;
use warp::Buf;
use futures_util::StreamExt;
use futures::future;

//  async fn run() {
//  let data = "--X-BOUNDARY\r\nContent-Disposition: form-data; \
//      name=\"my_text_field\"\r\n\r\nabcd\r\n--X-BOUNDARY--\r\n";

//  let stream = once(async move { Result::<Bytes, Infallible>::Ok(Bytes::from(data)) });
//  let mut multipart = Multipart::new(stream, "X-BOUNDARY");

//  while let Some(field) = multipart.next_field().await.unwrap() {
//      let content = field.text().await.unwrap();
//      assert_eq!(content, "abcd");
//  }
// }
// tokio::runtime::Runtime::new().unwrap().block_on(run());
pub struct NotCopy;

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
            let values_res = part.stream().then(|result| {
                let binding = result.unwrap();
                let slice = binding.chunk();
                let value_as_str = std::str::from_utf8(slice).unwrap();
                future::ok::<_, ()>(value_as_str.to_string())
            }).collect::<Vec<_>>().await;
            let value_str:String = values_res.into_iter().map(|x| x.unwrap()).collect();
            
            match name.as_str()  {
                "subject" => post.subject = value_str.to_string(),
                "text" => post.text = value_str.to_string(),
                "board_id" => post.board_id = value_str.to_string(),
                "name" => post.poster = value_str.to_string(),
                "op" => post.op = value_str.to_string(),
                _ => info!("name not found in form input {name}")//Err(warp::reject::custom(InvalidParameter))?,
            }
        }
        else {
        println!("Receiving file: {}", filename);

        let mut data = Vec::new();
        let mut stream = part.stream();
        
        while let Ok(Some(mut chunk)) = stream.try_next().await {
            let slice: &[u8] = chunk.chunk();
            data.extend_from_slice(slice);
            chunk.advance(slice.len());
        }

        let save_path = PathBuf::from(format!("/tmp/uploads/{}", filename));
        let filepath = save_path.to_str().unwrap().to_string();
        post.file = filepath.to_owned();
        post.file_name = filename.to_owned();
        post.file_size = data.len() as u64;
        post.file_original_name = filename.to_owned();
        let created_dir_result = tokio::fs::create_dir_all(PathBuf::from("/tmp/uploads")).await;
        match created_dir_result {
            Err(_) => return Ok(GenericResponse::new(warp::http::StatusCode::INTERNAL_SERVER_ERROR, "Failed to create files dir".to_string())),
            Ok(_) => { info!("Dir created successfully"); }
        }
        match tokio::fs::write(&save_path, &data).await {
            Err(_) => return Ok(GenericResponse::new(warp::http::StatusCode::INTERNAL_SERVER_ERROR, "Failed to save file".to_string())),
            Ok(_) => { info!("File saved successfully"); }
        };

        let file_dimensions = get_image_dimensions(filepath.as_str()).unwrap();
        post.file_dimensions = format!("{}x{}", file_dimensions.0, file_dimensions.1);

        
        // let s3_response = upload_to_s3(save_path).await.unwrap();
        // match s3_response {
        //     Ok(_) => { info!("File uploaded to S3 successfully"); }
        //     Err(_) => return Ok(GenericResponse::new(warp::http::StatusCode::INTERNAL_SERVER_ERROR, "Failed to upload file to S3".to_string())),
        // }
        
    }
    }
    post.ip = addr.unwrap().to_string();
    dbg!(&post);
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


// pub async fn upload_to_s3(path:PathRef) -> Result<rusoto_s3::PutObjectOutput, Box<dyn std::error::Error>> {
//     let bucket = std::env::var("S3_BUCKET").unwrap();
//     let region = std::env::var("S3_REGION").unwrap();
//     let client = rusoto_s3::S3Client::new(rusoto_core::Region::Custom {
//         name: region,
//         endpoint: "http://localhost:9000".to_string(),
//     });

//     let file = std::fs::File::open(path).unwrap();
//     let file_name = Path::new(&path).file_name().unwrap().to_str().unwrap();
//     let key = format!("uploads/{}", file_name);
//     let mut buffer = Vec::new();
//     file.take(1024 * 1024 * 25).read_to_end(&mut buffer).unwrap();

//     let request = rusoto_s3::PutObjectRequest {
//         bucket: bucket,
//         key: key,
//         body: Some(buffer.into()),
//         ..Default::default()
//     };

//     let response = client.put_object(request).await?;
//     Ok(response)
// }

pub async fn list_posts_by_board_handler(board_id: String) -> WebResult {
    info!("list_posts_by_board_handler:");
    let posts = list_posts_by_board(board_id).await.unwrap();

    let response = GenericResponse::new(warp::http::StatusCode::OK, posts);
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
