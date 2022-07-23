use actix_cors::Cors;
use actix_web::{web , get,
    http::header::ContentType,
    http,
    body::BoxBody,
    App,
    
     Responder, HttpResponse, HttpServer};

use serde::Serialize;
use rusqlite::{Connection, Result, params};

// This struct represents state
#[derive(Serialize)]
struct AppState {
    posts: Vec<Post>,
    db_path: String
}

#[derive(Serialize, Clone, Debug)]
struct Post {
    id: String,
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
    let db_path = &data.db_path;
    let conn = Connection::open(db_path).unwrap();

    let mut stmt = conn.prepare("SELECT id, title, content FROM posts WHERE ID = (?1) limit 1").expect(&format!("Could not find the post with id {}", id.to_string()));
    let mut posts = stmt.query_map(rusqlite::params![id.to_string()], |row| {
        Ok(Post {
            id:row.get("id")?,
            title : row.get("title")?,
            content: row.get("content")?
        })
    }).unwrap();


    let found_post = posts.next().unwrap().unwrap();

    found_post
}

pub fn create_db() -> String{
    let conn = Connection::open("./test_db.db").unwrap();
    let db_path = conn.path().unwrap().to_str().unwrap();
    conn.execute(
        "CREATE TABLE posts(
            id TEXT PRIMARY KEY,
            title TEXT NOT NULL,
            content TEXT NOT NULL
        )"
        , ()
    );

    let post1= Post {
        id: String::from("1"),
        title: String::from("hahah title"),
        content: String::from("This is the coolest")
    };
    let post2= Post {
        id: String::from("2"),
        title: String::from("Rust with Masai"),
        content: String::from("Programming is fun. Especially with Masai")
    };

    conn.execute(
        "INSERT INTO posts (ID,title, content) VALUES (?1, ?2, ?3)",
        (&post1.id, &post1.title, &post1.content)
    );
    conn.execute(
        "INSERT INTO posts (ID,title, content) VALUES (?1, ?2, ?3)",
        (&post2.id, &post2.title, &post2.content)
    );
   db_path.to_owned()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
   


    HttpServer::new(|| {
        let db_path = create_db();
    

        let cors = Cors::default()
            .allowed_origin(format!("http://localhost:8080").as_str())
            .allowed_methods(vec!["GET", "POST"])
            .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
            .allowed_header(http::header::CONTENT_TYPE)
            .max_age(3600);

            let all_posts = vec![
                Post {
                    id: String::from("1"),
                    title: String::from("hahah title"),
                    content: String::from("This is the coolest")
                },
                Post {
                    id: String::from("2"),
                    title: String::from("Masai vs Floyd Mayweather"),
                    content: String::from("Masai smashes Mayweather in London")
                }
            ];
        App::new()
        .wrap(cors)
        .app_data(web::Data::new(AppState {
            posts: all_posts,
            db_path: db_path.clone()
        }))
        .service(web::scope("/api")
        .service(blog_post)
        // .service(blog)
    )
    })

    .bind(("0.0.0.0", 8000))?
    .run()
    .await
}