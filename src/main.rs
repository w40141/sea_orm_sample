use actix_web::{get, App, HttpResponse, HttpServer};
use dotenv::dotenv;
use std::io;

// use sea_orm_sample::presentation::user::{register_user, registered};

// #[tokio::main]
#[actix_web::main]
async fn main() -> io::Result<()> {
    dotenv().ok();
    env_logger::init();

    HttpServer::new(
        || App::new().service(hello), // .service(registered)
    )
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

#[get("/")]
async fn hello() -> HttpResponse {
    HttpResponse::Ok().content_type("text/html").body(
        r#"
        hello
        "#,
    )
}
