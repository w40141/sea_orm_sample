use anyhow::Result;
use derive_new::new;
use sea_orm::prelude::ChronoDateTimeLocal;
use sea_orm::{ActiveModelTrait, Database, Set};

use crate::domain::user::{IUserRepository, User};
use crate::entity::user::ActiveModel;

use super::configure::connection;

#[derive(new)]
pub struct UserRepository {}

#[async_trait::async_trait]
impl IUserRepository for UserRepository {
    async fn insert(&self, user_domain: &User) -> Result<i64> {
        let db = connection().await?;
        let active_model = ActiveModel {
            name: Set(user_domain.name().to_owned()),
            email: Set(user_domain.email().into()),
            password: Set(user_domain.password().into()),
            profile: Set(user_domain.profile().to_owned()),
            created_at: Set(chrono::Utc::now()),
            updated_at: Set(chrono::Utc::now()),
            ..Default::default()
        };
        let user_model = active_model.insert(&db).await?;
        Ok(user_model)
    }

    async fn find_by_name(&self, name: String) -> Result<User> {
        todo!()
    }
    async fn find_by_email(&self, email: String) -> Result<User> {
        todo!()
    }
    async fn delete(&self, user: &User) -> Result<()> {
        todo!()
    }
    async fn update_name(&self, user: &User, name: String) -> Result<()> {
        todo!()
    }
    async fn update_email(&self, user: &User, email: String) -> Result<()> {
        todo!()
    }
    async fn update_profile(&self, user: &User, profile: String) -> Result<()> {
        todo!()
    }
    async fn update_password(&self, user: &User, password: String) -> Result<()> {
        todo!()
    }
    async fn list(&self) -> Result<Vec<User>> {
        todo!()
    }
}
