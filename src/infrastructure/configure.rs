use anyhow::{anyhow, Result};
use dotenv::dotenv;
use sea_orm::{Database, DatabaseConnection};
use std::env;

pub async fn get_db_url() -> String {
    dotenv().ok();
    env::var("DATABASE_URL").expect("Can't get DB URL")
}

pub async fn connection() -> Result<DatabaseConnection> {
    let url = get_db_url().await;
    match Database::connect(url).await {
        Ok(c) => Ok(c),
        Err(e) => Err(anyhow!(e)),
    }
}
