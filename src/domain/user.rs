use anyhow::Result;
use async_trait::async_trait;
use derive_builder::Builder;
use derive_new::new;
use getset::Getters;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[async_trait]
pub trait IUserRepository {
    async fn insert(&self, user: &User) -> Result<u64>;
    async fn find_by_id(&self, id: &u64) -> Result<Option<(u64, User)>>;
    async fn find_by_name(&self, name: &String) -> Result<Option<(u64, User)>>;
    async fn find_by_email(&self, email: &String) -> Result<Option<(u64, User)>>;
    async fn delete_by_id(&self, id: &u64, password: &String) -> Result<()>;
    async fn delete_by_name(&self, name: &String, password: &String) -> Result<()>;
    async fn update_name(
        &self,
        old_name: &String,
        password: &String,
        new_name: &String,
    ) -> Result<()>;
    async fn update_email(
        &self,
        old_email: &String,
        password: &String,
        new_email: &String,
    ) -> Result<()>;
    // async fn update_password(&self, user: &User, password: String) -> Result<()>;
    async fn list(&self) -> Result<Vec<User>>;
    async fn exist_by_id(&self, id: &u64) -> Result<bool> {
        let f = match self.find_by_id(&id).await? {
            Some(_) => true,
            None => false,
        };
        Ok(f)
    }
    async fn exist_by_name(&self, name: &String) -> Result<bool> {
        let f = match self.find_by_name(&name).await? {
            Some(_) => true,
            None => false,
        };
        Ok(f)
    }
    async fn exist_by_email(&self, email: &String) -> Result<bool> {
        let f = match self.find_by_email(&email).await? {
            Some(_) => true,
            None => false,
        };
        Ok(f)
    }
}

#[derive(Debug, Serialize, Deserialize, new, Getters, Clone, Builder, PartialEq)]
#[getset(get = "pub")]
#[builder(setter(into))]
pub struct User {
    name: String,
    email: Email,
    password: Password,
    #[builder(default)]
    profile: Option<String>,
    enable: bool,
}

#[derive(Debug, Serialize, Deserialize, new, Getters, Clone, Validate, PartialEq)]
#[getset(get = "pub")]
pub struct Email {
    #[validate(email)]
    email: String,
}

#[derive(Debug, Serialize, Deserialize, new, Getters, Clone, Validate, PartialEq)]
#[getset(get = "pub")]
pub struct Password {
    #[validate(length(min = 8))]
    password: String,
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn password_test() {
        {
            let password = Password::new("1234".to_string());
            assert_eq!(password.validate().is_err(), true);
        }
        {
            let password = Password::new("12345678".to_string());
            assert_eq!(password.validate().is_ok(), true);
        }
    }

    #[test]
    fn email_test() {
        {
            let email = Email::new("1234".to_string());
            assert_eq!(email.validate().is_err(), true);
        }
        {
            let email = Email::new("12345678@test.com".to_string());
            assert_eq!(email.validate().is_ok(), true);
        }
    }

    #[test]
    fn user_test() {
        {
            let test_user_0: Result<User, UserBuilderError> = UserBuilder::default()
                .email(Email::new("test@test.com".to_string()))
                .name("daisuke")
                .password(Password::new("12345678".to_string()))
                .enable(true)
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
                    profile: None,
                    enable: true,
                }
            );
        }
        {
            let test_user_0: Result<User, UserBuilderError> = UserBuilder::default()
                .email(Email::new("testtest.com".to_string()))
                .name("daisuke")
                .password(Password::new("123456".to_string()))
                .enable(true)
                .build();
            assert_eq!(&test_user_0.is_ok(), &true);
            let user = test_user_0.unwrap();
            assert_eq!(
                user,
                User {
                    email: Email::new("testtest.com".to_string()),
                    name: "daisuke".to_string(),
                    password: Password::new("123456".to_string()),
                    profile: None,
                    enable: true,
                }
            );
        }
    }
}
