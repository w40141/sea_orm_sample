use anyhow::Result;
use derive_new::new;
use getset::Getters;
use serde::{Deserialize, Serialize};

use crate::application::user::IUserRepository;
use crate::domain::user::User;

#[derive(Debug, Serialize, Deserialize, new, Getters)]
#[getset(get = "pub")]
pub struct UserRepository {
    user: User,
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
