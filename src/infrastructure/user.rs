use actix_web::Result;
use derive_new::new;
// use dotenv::dotenv;
// use sea_orm::Database;
// use std::env;

// dotenv().ok();
// let database = env::var("DATABASE_URL")?;
// let _ = Database::connect(database).await?;
// println!("hello");
// Ok(())

use crate::application::user::IUserRepository;
use crate::domain::user::User;

#[derive(new)]
pub struct UserRepository {}

#[async_trait::async_trait]
impl IUserRepository for UserRepository {
    async fn register_user_into_db(&self, domain: &User) -> Result<Option<User>> {
        todo!()
    }

    async fn delete_user_from_db(&self, domain: &User) -> Result<Option<User>> {
        todo!()
    }

    async fn change_name_in_db(&self, domain: &User) -> Result<Option<User>> {
        todo!()
    }

    async fn change_email_in_db(&self, domain: &User) -> Result<Option<User>> {
        todo!()
    }
}
