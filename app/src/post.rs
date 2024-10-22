use crate::GenericResponse;
use crate::WebResult;
use bytes::BufMut;
use uuid::Uuid;
use futures_util::TryStreamExt;
use warp::filters::addr;
use warp::filters::BoxedFilter;
use warp::{Reply, Buf,Filter};
use warp::multipart::FormData;
use std::fs;
use std::net::SocketAddr;
use warp::reply::json;
use tracing::{error, info};
use crate::CONTENT_LIMIT;
use crate::model::Post;
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
          let field_name = p.name();{
            let filename = p.filename().to_owned();
            let filepath = format!("/tmp/uploads/{}.{}", Uuid::new_v4(),get_extension_from_filename(filename.to_string()));
            info!("file field {}: filename {:?}", field_name, filename);

            // validate mime types
            let mime = p.content_type().unwrap();
            if !mime.starts_with("image") {
                error!("invalid mime type: {}", mime);
                return Err(warp::reject::reject());
            }

            // copy to /tmp dir
            fs::create_dir_all("/tmp/uploads").unwrap();
            save_part_to_file(&filepath, p).await.expect("save error");
            post.file = filepath.to_owned();
            post.file_name = filename.unwrap().to_string();
            post.file_size = fs::metadata(&filepath).unwrap().len();
            post.file_original_name = filename.unwrap().to_string();
          }
         } else {
          let field_name = p.name().to_owned();{
            info!("not file field {}:", field_name);
            let field_value = p.stream()
                .try_fold(Vec::new(), |mut vec, data| {
                    vec.put(data);
                    async move { Ok(vec) }
                })
                .await
                .map_err(|e| {
                          error!("reading file error: {}", e);
                          warp::reject::reject()
                      })?;
            let value_string = String::from_utf8(field_value);
            post.ip = addr.unwrap().to_string();
            match field_name.as_str() {
              "subject"  => post.subject = value_string.unwrap().to_string(),
              "text"=> post.text = value_string.unwrap().to_string(),
              "board"=> post.board = value_string.unwrap().to_string(),
              "poster"=> post.poster = value_string.unwrap().to_string(),
              "op"=> post.op = value_string.unwrap().to_string(),
            }
            info!("field {:?}: {:?}", field_name, value_string);
          }
          }
         
        // } else {
        //   let field_name = unwrapped_p.name();{
        //     info!("field {}:", field_name);
        //   //   let field_name = unwrapped_p.name();{
        //   //     let mut final_value:String = String::new();
        //   //     {
        //   //   let field_value = unwrapped_p
        //   //   .stream()
        //   //   .try_fold(Vec::new(), |mut vec, data| {
        //   //       vec.put(data);
        //   //       async move { Ok(vec) }
        //   //   })
        //   //   .await
        //   //   .map_err(|e| {
        //   //       error!("reading file error: {}", e);
        //   //       warp::reject::reject()
        //   //   })?;
        //   //   let utf8_value = match String::from_utf8(field_value) {
        //   //       Ok(v) => v,
        //   //       Err(e) => {
        //   //           error!("error parsing field value: {}", e);
        //   //           return Err(warp::reject::reject());
        //   //       }
        //   //   };
        //   //   final_value = utf8_value;
          
        //   //   info!("field {}: {}", field_name, final_value);
        //   // }
        //   }
        // }
    }


    let message: String = format!("field_names ");

    let response_json = &GenericResponse {
        status: "success".to_string(),
        message,
    };
    info!("Response: {:?}", response_json);
    Ok(json(response_json))
}


async fn save_part_to_file(path: &str, part: warp::multipart::Part) -> Result<(), std::io::Error> {
  let data = part
      .stream()
      .try_fold(Vec::new(), |mut acc, buf| async move {
          acc.extend_from_slice(buf.chunk());
          Ok(acc)
      })
      .await.expect("folding error");
  std::fs::write(path, data)
}

fn get_extension_from_filename(filename: String) -> String {

  //Change it to a canonical file path.
  let path = Path::new(&filename).canonicalize().expect(
      "Expecting an existing filename",
  );

  let filepath = path.to_str();
  let name = filepath.unwrap().split('/');
  let names: Vec<&str> = name.collect();
  let extension = names.last().expect("File extension can not be read.");
  let extens: Vec<&str> = extension.split(".").collect();

  extens[1..(extens.len())].join(".").to_string()
}