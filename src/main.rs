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


#[actix_web::put("/posts/{id}")]
async fn update_post(
    data: web::Data<AppState>,
    post_id: web::Path<usize>,
    updated_post: web::Json<Post>,
) -> impl Responder {
    let mut posts = data.posts.lock().unwrap();
    if let Some(post) = posts.iter_mut().find(|p| p.id == post_id.into_inner()) {
        post.title = updated_post.title.clone();
        post.content = updated_post.content.clone();
        HttpResponse::Ok().json(post)
    } else {
        HttpResponse::NotFound().body("Post not found")
    }
}

#[actix_web::delete("/posts/{id}")]
async fn delete_post(data: web::Data<AppState>, post_id: web::Path<usize>) -> impl Responder {
    let mut posts = data.posts.lock().unwrap();
    if posts.iter().any(|p| p.id == post_id.into_inner()) {
        posts.retain(|p| p.id != post_id.into_inner());
        HttpResponse::Ok().body("Post deleted")
    } else {
        HttpResponse::NotFound().body("Post not found")
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let app_data = web::Data::new(AppState {
        posts: Mutex::new(Vec::new()),
    });

    HttpServer::new(move || {
        App::new()
            .app_data(app_data.clone())
            .service(get_posts)
            .service(create_post)
            .service(update_post)
            .service(delete_post)
    })
    .bind("127.0.0.1:5000")?
    .run()
    .await
}
