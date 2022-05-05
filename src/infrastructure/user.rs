use anyhow::{anyhow, Ok, Result};
use derive_new::new;
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter, Set};

use super::configure::connection;
use crate::domain::user::{Email, IUserRepository, Password, User, UserBuilder};
use crate::entity::user::{self, ActiveModel, Entity as UserEntity};

#[derive(new)]
pub struct UserRepository {}

#[async_trait::async_trait]
impl IUserRepository for UserRepository {
    async fn insert(&self, user_domain: &User) -> Result<u64> {
        let db = connection().await?;
        let active_model = ActiveModel {
            name: Set(user_domain.name().to_string()),
            email: Set(user_domain.email().email().to_string()),
            password: Set(user_domain.password().password().to_string()),
            profile: Set(user_domain.profile().to_owned()),
            created_at: Set(chrono::Utc::now()),
            updated_at: Set(chrono::Utc::now()),
            ..Default::default()
        };
        let user_model = active_model.insert(&db).await?;
        Ok(user_model.id)
    }

    async fn find_by_id(&self, id: &u64) -> Result<Option<(u64, User)>> {
        let db = connection().await?;
        let model = UserEntity::find_by_id(id.to_owned()).one(&db).await?;
        let m = match model {
            Some(m) => Some((
                m.id,
                UserBuilder::default()
                    .name(m.name)
                    .email(Email::new(m.email))
                    .password(Password::new(m.password))
                    .build()?,
            )),
            _ => None,
        };
        Ok(m)
    }

    async fn find_by_name(&self, name: &String) -> Result<Option<(u64, User)>> {
        let db = connection().await?;
        let model = UserEntity::find()
            .filter(user::Column::Name.eq(name.clone()))
            .one(&db)
            .await?;
        let m = match model {
            Some(m) => Some((
                m.id,
                UserBuilder::default()
                    .name(m.name)
                    .email(Email::new(m.email))
                    .password(Password::new(m.password))
                    .build()?,
            )),
            _ => None,
        };
        Ok(m)
    }

    async fn find_by_email(&self, email: &String) -> Result<Option<(u64, User)>> {
        let db = connection().await?;
        let model = UserEntity::find()
            .filter(user::Column::Email.eq(email.clone()))
            .one(&db)
            .await?;
        let m = match model {
            Some(m) => Some((
                m.id,
                UserBuilder::default()
                    .name(m.name)
                    .email(Email::new(m.email))
                    .password(Password::new(m.password))
                    .build()?,
            )),
            _ => None,
        };
        Ok(m)
    }

    async fn delete_by_id(&self, id: &u64, password: &String) -> Result<()> {
        let db = connection().await?;
        let user = self.find_by_id(id).await?;
        match user {
            Some((_, u)) if u.password().password() == password => {
                let _ = UserEntity::delete_by_id(id.clone()).exec(&db).await?;
                Ok(())
            }
            Some(_) => Err(anyhow!("Password is incorrect.")),
            _ => Err(anyhow!("Not found {id}.")),
        }
    }

    async fn delete_by_name(&self, name: &String, password: &String) -> Result<()> {
        let db = connection().await?;
        let user = self.find_by_name(&name).await?;
        match user {
            Some((i, u)) if u.password().password() == password => {
                let _ = UserEntity::delete_by_id(i).exec(&db).await?;
                Ok(())
            }
            Some(_) => Err(anyhow!("Password is incorrect.")),
            _ => Err(anyhow!("Not found {name}.")),
        }
    }

    async fn update_name(
        &self,
        old_name: &String,
        password: &String,
        new_name: &String,
    ) -> Result<()> {
        let db = connection().await?;
        let model = UserEntity::find()
            .filter(user::Column::Name.eq(old_name.clone()))
            .one(&db)
            .await?;
        match model {
            Some(m) if m.password == password.to_string() => {
                let mut active_model: ActiveModel = m.into();
                active_model.name = Set(new_name.to_owned());
                let _ = active_model.update(&db).await?;
                Ok(())
            }
            Some(_) => Err(anyhow!("Password is incorrect.")),
            _ => Err(anyhow!("Not found {old_name}.")),
        }
    }

    async fn update_email(
        &self,
        old_email: &String,
        password: &String,
        new_email: &String,
    ) -> Result<()> {
        let db = connection().await?;
        let model = UserEntity::find()
            .filter(user::Column::Email.eq(old_email.clone()))
            .one(&db)
            .await?;
        match model {
            Some(m) if m.password == password.to_string() => {
                let mut active_model: ActiveModel = m.into();
                active_model.email = Set(new_email.to_owned());
                let _ = active_model.update(&db).await?;
                Ok(())
            }
            Some(_) => Err(anyhow!("Password is incorrect.")),
            _ => Err(anyhow!("Not found {old_email}.")),
        }
    }

    async fn list(&self) -> Result<Vec<User>> {
        let db = connection().await?;
        let models = UserEntity::find_by_id(1).all(&db).await?;
        Ok(models
            .into_iter()
            .filter_map(|m| {
                UserBuilder::default()
                    .name(m.name)
                    .email(Email::new(m.email))
                    .password(Password::new(m.password))
                    .build()
                    .ok()
            })
            .collect())
    }
}
