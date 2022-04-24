use anyhow::Result;
use dotenv::dotenv;
use sea_orm::{Database, DatabaseConnection};
use std::env;

pub async fn get_db_url() -> String {
    dotenv().ok();
    env::var("DATABASE_URL").expect("Can't get DB URL")
}

// pub async fn connection() -> DatabaseConnection {
//     Database::connect(get_db_url()).await?
// }
