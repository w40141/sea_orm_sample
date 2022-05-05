use anyhow::{anyhow, Result};
use derive_new::new;
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter, QueryOrder, Set};
use validator::Validate;

use super::configure::connection;
use crate::domain::post::{IPostRepository, Post, PostBuilder, PostContent};
use crate::entity::post::{self, ActiveModel, Entity as PostEntity};

#[derive(new)]
pub struct PostRepository {}

#[async_trait::async_trait]
impl IPostRepository for PostRepository {
    async fn insert(&self, post_domain: &Post) -> Result<u64> {
        let db = connection().await?;
        {
            post_domain.content().validate()?;
        }
        let active_model = ActiveModel {
            content: Set(post_domain.content().content().to_string()),
            user_id: Set(post_domain.user_id().to_owned()),
            created_at: Set(chrono::Utc::now()),
            updated_at: Set(chrono::Utc::now()),
            ..Default::default()
        };
        let model = active_model.insert(&db).await?;
        Ok(model.id)
    }

    async fn find_by_id(&self, id: &u64) -> Result<Option<(u64, Post)>> {
        let db = connection().await?;
        let model = PostEntity::find_by_id(id.to_owned()).one(&db).await?;
        let m = if let Some(m) = model {
            Some((
                m.id,
                PostBuilder::default()
                    .user_id(m.user_id)
                    .content(PostContent::new(m.content))
                    .build()?,
            ))
        } else {
            None
        };
        Ok(m)
    }

    async fn find_by_user_id(&self, user_id: &u64) -> Result<Vec<Post>> {
        let db = connection().await?;
        let models = PostEntity::find()
            .filter(post::Column::UserId.eq(user_id.clone()))
            .order_by_asc(post::Column::CreatedAt)
            .all(&db)
            .await?;
        Ok(models
            .into_iter()
            .filter_map(|m| {
                PostBuilder::default()
                    .user_id(m.user_id)
                    .content(PostContent::new(m.content))
                    .build()
                    .ok()
            })
            .collect())
    }

    async fn find_by_user_name(&self, name: &String) -> Result<Option<(u64, Post)>> {
        todo!()
    }

    async fn list(&self) -> Result<Vec<Post>> {
        let db = connection().await?;
        let models = PostEntity::find().all(&db).await?;
        Ok(models
            .into_iter()
            .filter_map(|m| {
                PostBuilder::default()
                    .user_id(m.user_id)
                    .content(PostContent::new(m.content))
                    .build()
                    .ok()
            })
            .collect())
    }
}
