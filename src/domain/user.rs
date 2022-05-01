use async_trait::async_trait;
use derive_builder::Builder;
use derive_new::new;
use getset::Getters;
use serde::{Deserialize, Serialize};
use validator::Validate;
// #[async_trait]
// pub trait IUserRepository {
//     async fn find_by_name(&self, name: String) -> Result<User>;
//     async fn find_by_email(&self, email: String) -> Result<User>;
//     async fn insert(&self, user: &User) -> Result<()>;
//     async fn delete(&self, user: &User) -> Result<()>;
//     async fn update_name(&self, user: &User, name: String) -> Result<()>;
//     async fn update_email(&self, user: &User, email: String) -> Result<()>;
//     async fn update_profile(&self, user: &User, profile: String) -> Result<()>;
//     async fn update_password(&self, user: &User, password: String) -> Result<()>;
//     async fn list(&self) -> Result<Vec<User>>;
//     async fn exist_by_name(&self, name: String) -> bool {
//         match self.find_by_name(name).await {
//             Ok(_) => true,
//             _ => false,
//         }
//     }
//     async fn exist_by_email(&self, email: String) -> bool {
//         match self.find_by_email(email).await {
//             Ok(_) => true,
//             _ => false,
//         }
//     }
// }
#[derive(Debug, Serialize, Deserialize, new, Getters, Clone, Builder, Validate)]
#[getset(get = "pub")]
#[builder(setter(into))]
pub struct User {
    #[builder(default)]
    id: Option<i32>,
    // #[builder(setter(into))]
    #[validate(email)]
    email: String,
    // #[builder(setter(into))]
    #[validate(length(min = 3))]
    name: String,
    // #[builder(setter(into))]
    #[validate(length(min = 8))]
    password: String,
    #[builder(default)]
    profile: Option<String>,
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn user_test() {
        let test_user_0: Result<User, UserBuilderError> = UserBuilder::default()
            .email("test@test.com")
            .name("daisuke")
            .password("0123456")
            .build();
        // println!("{:?}", &test_user_0.unwrap());
        assert_eq!(&test_user_0.is_ok(), &true);
    }
}
