use std::env;

use dotenv::dotenv;
use sea_orm::Database;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    let database = env::var("DATABASE_URL")?;
    let _ = Database::connect(database).await?;
    println!("hello");
    Ok(())
}
