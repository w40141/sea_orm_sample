use anyhow::Result;
use async_trait::async_trait;
use derive_builder::Builder;
use derive_new::new;
use getset::Getters;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[async_trait]
pub trait IUserRepository {
    async fn find_by_name(&self, name: String) -> Result<User>;
    async fn find_by_email(&self, email: String) -> Result<User>;
    async fn insert(&self, user: &User) -> Result<i64>;
    async fn delete(&self, user: &User) -> Result<()>;
    async fn update_name(&self, user: &User, name: String) -> Result<()>;
    async fn update_email(&self, user: &User, email: String) -> Result<()>;
    async fn update_profile(&self, user: &User, profile: String) -> Result<()>;
    async fn update_password(&self, user: &User, password: String) -> Result<()>;
    async fn list(&self) -> Result<Vec<User>>;
    async fn exist_by_name(&self, name: String) -> bool {
        match self.find_by_name(name).await {
            Ok(_) => true,
            _ => false,
        }
    }
    async fn exist_by_email(&self, email: String) -> bool {
        match self.find_by_email(email).await {
            Ok(_) => true,
            _ => false,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, new, Getters, Clone, Builder, Validate, PartialEq)]
#[getset(get = "pub")]
#[builder(setter(into))]
pub struct User {
    name: String,
    email: Email,
    password: Password,
    #[builder(default)]
    profile: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, new, Validate, PartialEq)]
pub struct Email {
    #[validate(email)]
    email: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, new, Validate, PartialEq)]
pub struct Password {
    #[validate(length(min = 8))]
    password: String,
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn user_test() {
        {
            let test_user_0: Result<User, UserBuilderError> = UserBuilder::default()
                .email(Email::new("test@test.com".to_string()))
                .name("daisuke")
                .password(Password::new("12345678".to_string()))
                .build();
            // println!("{:?}", &test_user_0.unwrap().as_ref());
            assert_eq!(&test_user_0.is_ok(), &true);
            let user = test_user_0.unwrap();
            assert_eq!(
                user,
                User {
                    email: Email::new("test@test.com".to_string()),
                    name: "daisuke".to_string(),
                    password: Password::new("12345678".to_string()),
                    profile: None
                }
            );
            assert_eq!(user.password().validate().is_ok(), true);
            assert_eq!(user.email().validate().is_ok(), true);
        }
        {
            let test_user_0: Result<User, UserBuilderError> = UserBuilder::default()
                .email(Email::new("testtest.com".to_string()))
                .name("daisuke")
                .password(Password::new("123456".to_string()))
                .build();
            assert_eq!(&test_user_0.is_ok(), &true);
            let user = test_user_0.unwrap();
            assert_eq!(
                user,
                User {
                    email: Email::new("testtest.com".to_string()),
                    name: "daisuke".to_string(),
                    password: Password::new("123456".to_string()),
                    profile: None
                }
            );
            assert_eq!(user.password().validate().is_err(), true);
            assert_eq!(user.email().validate().is_err(), true);
        }
    }
}
