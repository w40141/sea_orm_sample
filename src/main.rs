use actix_web::{App, HttpServer};
use std::io;
// use dotenv::dotenv;
// use sea_orm::Database;
// use std::env;

#[tokio::main]
async fn main() -> io::Result<()> {
    HttpServer::new(move || App::new().service(register_user))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
    // dotenv().ok();
    // let database = env::var("DATABASE_URL")?;
    // let _ = Database::connect(database).await?;
    // println!("hello");
    // Ok(())
}
