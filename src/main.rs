use sea_orm::Database;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let _ = Database::connect("mysql://root:root@localhost:3306/store").await?;
    println!("hello");
    Ok(())
}
