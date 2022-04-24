use anyhow::{Ok, Result};
use async_trait::async_trait;
use derive_new::new;
use std::sync::Arc;

use crate::domain::user::User;
use crate::infrastructure::user::UserRepository;
use crate::presentation::user::IUserService;

#[async_trait]
pub trait IUserRepository {
    async fn register(&self) -> Result<Option<User>>;
    async fn delete(&self) -> Result<Option<User>>;
    async fn change_name(&self) -> Result<Option<User>>;
    async fn change_email(&self) -> Result<Option<User>>;
}

#[derive(new)]
pub struct UserService {
    repository: Arc<dyn UserRepository + Sync + Send>,
}

#[async_trait]
impl IUserService for UserService {
    async fn register(&self) -> Result<Option<User>> {
        Ok(self.repository.register().await?)
    }
}

// #[derive(new)]
// pub struct DeleteUserHandler {
//     user_mantle: Arc<dyn UserDomain + Sync + Send>,
// }
//
// impl DeleteUserHandler {
//     pub async fn execute(&self, user: User) -> Result<User> {
//         Ok(self.user_mantle.delete(&user).await?)
//     }
// }
//
// #[derive(new)]
// pub struct ChangeNameHandler {
//     user_mantle: Arc<dyn UserDomain + Sync + Send>,
// }
//
// impl ChangeNameHandler {
//     pub async fn execute(&self, user: User) -> Result<User> {
//         Ok(self.user_mantle.change_name(&user).await?)
//     }
// }
//
// #[derive(new)]
// pub struct ChangeEmailHandler {
//     user_mantle: Arc<dyn UserDomain + Sync + Send>,
// }
//
// impl ChangeEmailHandler {
//     pub async fn execute(&self, user: User) -> Result<User> {
//         Ok(self.user_mantle.change_email(&user).await?)
//     }
// }
