use crate::model::{Board, Post};
use serde::Serialize;

#[derive(Serialize)]
pub struct GenericResponse {
    pub status: String,
    pub message: String,
}

#[derive(Serialize, Debug)]
pub struct PostData {
    pub post: Post,
}

#[derive(Serialize, Debug)]
pub struct PostListResponse {
    pub status: String,
    pub results: usize,
    pub posts: Vec<Post>,
}

#[derive(Serialize, Debug)]
pub struct BoardListResponse {
    pub status: String,
    pub results: usize,
    pub boards: Vec<Board>,
}