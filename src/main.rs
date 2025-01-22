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

#[actix_web::get("/posts")]
async fn get_posts(data: web::Data<AppState>) -> impl Responder {
    let posts = data.posts.lock().unwrap();
    HttpResponse::Ok().json(&*posts)
}

#[actix_web::post("/posts")]
async fn create_post(data: web::Data<AppState>, new_post: web::Json<Post>) -> impl Responder {
    let mut posts = data.posts.lock().unwrap();
    posts.push(Post {
        id: posts.len() + 1,
        title: new_post.title.clone(),
        content: new_post.content.clone(),
    });
    HttpResponse::Ok().json(&*posts)
}
