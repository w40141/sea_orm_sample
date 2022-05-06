use anyhow::Result;
use async_trait::async_trait;
use derive_new::new;
use std::sync::Arc;

use crate::domain::user::{IUserRepository, User};
// use crate::presentation::user::IUserService;

// #[async_trait]
// pub trait IUserDomain {
//     async fn register_user_into_db(&self) -> Result<User>;
//     async fn delete_user_from_db(&self, domain: &User) -> Result<User>;
//     async fn change_name_in_db(&self, domain: &User) -> Result<User>;
//     async fn change_email_in_db(&self, domain: &User) -> Result<User>;
// }

#[derive(new)]
pub struct UserService<T>
where
    T: IUserRepository,
{
    repository: T,
}

// #[async_trait]
// impl IUserService for UserService {
impl<T: IUserRepository> UserService<T> {
    // async fn register_user_service(&self, domain: &User) -> Result<User> {
    //     Ok(self.repository.register_user_into_db(domain).await?)
    // }
    // async fn delete_user_service(&self, domain: &User) -> Result<User> {
    //     Ok(self.repository.delete_user_from_db(domain).await?)
    // }
    // async fn change_user_name_service(&self, domain: &User) -> Result<User> {
    //     Ok(self.repository.change_name_in_db(domain).await?)
    // }
    // async fn change_user_email_service(&self, domain: &User) -> Result<User> {
    //     Ok(self.repository.change_email_in_db(domain).await?)
    // }
}
