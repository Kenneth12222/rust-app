use actix_web::{web, App, HttpServer, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use std::sync::Mutex;

#[derive(Serialize, Deserialize)]
struct Post {
    id: usize,
    title: String,
    content: String,
}

struct AppState {
    posts: Mutex<Vec<Post>>,
}
