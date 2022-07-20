use actix_cors::Cors;
use actix_web::{web , get,
     http::header::ContentType,
     http,
     body::BoxBody,
     App,
     
      Responder, HttpResponse, HttpServer};


use serde::Serialize;

// This struct represents state
#[derive(Serialize)]
struct AppState {
    posts: Vec<Post>,
}

#[derive(Serialize, Clone)]
struct Post {
    title: String,
    content: String
}

// Responder
impl Responder for Post {
    type Body = BoxBody;

    fn respond_to(self, req: &actix_web::HttpRequest) -> HttpResponse<Self::Body> {
        let body = serde_json::to_string(&self).unwrap();

        HttpResponse::Ok()
        .content_type(ContentType::json())
        .body(body)
    }
}

impl Responder for AppState {
    type Body = BoxBody;

    fn respond_to(self, req: &actix_web::HttpRequest) -> HttpResponse<Self::Body> {
        let body = serde_json::to_string(&self).unwrap();

        HttpResponse::Ok()
        .content_type(ContentType::json())
        .body(body)
    }
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey champ. You found me")
}

#[get("/blog/{id}")]
async fn blog_post(id: web::Path<usize>, data : web::Data<AppState>) -> impl Responder {
    let first = &data.posts.get(0);
    let x = first.unwrap();
    Post {
        title: x.title.clone(),
        content: x.content.clone()
    }
}

#[get("/blog")]
async fn blog( data : web::Data<AppState>) -> impl Responder {
    let all_posts = &data;
    // let x = first.unwrap();
    // Post {
    //     title: x.title.clone(),
    //     content: x.content.clone()
    // }
    AppState{
        posts: all_posts.posts.clone()
    }
    
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
    

        let cors = Cors::default()
            .allowed_origin(format!("http://localhost:8080").as_str())
            .allowed_methods(vec!["GET", "POST"])
            .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
            .allowed_header(http::header::CONTENT_TYPE)
            .max_age(3600);

        App::new()
        .wrap(cors)
        .app_data(web::Data::new(AppState {
            posts: vec![
                Post {
                    title: String::from("hahah title"),
                    content: String::from("This is the coolest")
                },
                Post {
                    title: String::from("Masai vs Floyd Mayweather"),
                    content: String::from("Masai smashes Mayweather in London")
                }
            ]
        }))
        .service(web::scope("/api")
        .service(blog_post)
        .service(blog))
        .route("/hey", web::get().to(manual_hello))
    })

    .bind(("0.0.0.0", 8000))?
    .run()
    .await
}