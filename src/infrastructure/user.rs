use anyhow::Result;
use derive_new::new;
use sea_orm::prelude::ChronoDateTimeLocal;
use sea_orm::{ActiveModelTrait, Database, Set};

use crate::domain::user::IUserRepository;
use crate::domain::user::User;
use crate::entity::user::{ActiveModel, Model};

use super::configure::connection;

#[derive(new)]
pub struct UserRepository {}

#[async_trait::async_trait]
impl IUserRepository for UserRepository {
    async fn insert(&self, domain: &User) -> Result<User> {
        let db = connection().await;
        let user = ActiveModel {
            name: Set(domain.name().to_owned()),
            email: Set(domain.email().to_owned()),
            created_at: Set(chrono::Utc::now()),
            updated_at: Set(chrono::Utc::now()),
            ..Default::default()
        };
        let user: Model = user.insert(&db).await?;
        Ok(User::new(
            Some(user.id),
            user.name,
            user.email,
            "aaa",
            "aaaa",
        ))
    }

    async fn delete_user_from_db(&self, domain: &User) -> Result<User> {
        todo!()
    }

    async fn change_name_in_db(&self, domain: &User) -> Result<User> {
        todo!()
    }

    async fn change_email_in_db(&self, domain: &User) -> Result<User> {
        todo!()
    }
}
