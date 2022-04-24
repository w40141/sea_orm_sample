use anyhow::Result;
use derive_new::new;

use crate::domain::user::User;
use std::sync::Arc;

pub trait IUserApplication {
    async fn register(&self, user: &User) -> Result<Option<User>>;
    // async fn delete(&self, user: &User) -> Result<Option<User>>;
    // async fn change_name(&self, user: &User) -> Result<Option<User>>;
    // async fn change_email(&self, user: &User) -> Result<Option<User>>;
}

#[derive(new)]
pub struct RegisterUserHandler {
    user: User,
}

#[async_trait::async_trait]
pub trait IUserRepository {
    async fn register(&self, user: &User) -> Result<Option<User>>;
    async fn delete(&self, user: &User) -> Result<Option<User>>;
    async fn change_name(&self, user: &User) -> Result<Option<User>>;
    async fn change_email(&self, user: &User) -> Result<Option<User>>;
}

impl RegisterUserHandler {
    pub async fn execute(&self, user: User) -> Result<User> {
        Ok(self.user_domain.register(&user).await?)
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
