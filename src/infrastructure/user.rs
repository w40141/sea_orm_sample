use anyhow::Result;
use derive_new::new;

use crate::application::user::IUserRepository;
use crate::domain::user::User;

#[derive(new)]
pub struct UserRepository {
    domain: User,
}

#[async_trait::async_trait]
impl IUserRepository for UserRepository {
    async fn register(&self) -> Result<Option<User>> {
        todo!()
    }

    async fn delete(&self) -> Result<Option<User>> {
        todo!()
    }

    async fn change_name(&self) -> Result<Option<User>> {
        todo!()
    }

    async fn change_email(&self) -> Result<Option<User>> {
        todo!()
    }
}
