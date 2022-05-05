use anyhow::Result;
use derive_new::new;
use migration::JoinType;

use sea_orm::prelude::DateTimeUtc;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, EntityTrait, FromQueryResult, QueryFilter, QueryOrder,
    QuerySelect, RelationTrait, Set,
};
use validator::Validate;

use super::configure::connection;
use crate::domain::post::{IPostRepository, Post, PostBuilder, PostContent};
use crate::entity::post::{self, ActiveModel, Entity as PostEntity};
use crate::entity::user;

#[derive(new)]
pub struct PostRepository {}

#[derive(FromQueryResult)]
struct PostWithUser {
    id: u64,
    name: String,
    content: String,
    user_id: u64,
    enable: i8,
    created_at: DateTimeUtc,
    updated_at: DateTimeUtc,
}

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

    async fn find_by_user_id(&self, user_id: &u64) -> Result<Vec<Post>> {
        let db = connection().await?;
        let models = PostEntity::find()
            .column_as(user::Column::Name, "name")
            .join(JoinType::InnerJoin, user::Relation::Post.def())
            .filter(post::Column::UserId.eq(user_id.clone()))
            .order_by_asc(post::Column::CreatedAt)
            .into_model::<PostWithUser>()
            .all(&db)
            .await?;
        Ok(models
            .into_iter()
            .filter_map(|m| {
                PostBuilder::default()
                    .user_id(m.user_id)
                    .name(m.name)
                    .content(PostContent::new(m.content))
                    .build()
                    .ok()
            })
            .collect())
    }

    async fn find_by_user_name(&self, name: &String) -> Result<Vec<Post>> {
        let db = connection().await?;
        let models = PostEntity::find()
            .join(JoinType::InnerJoin, user::Relation::Post.def())
            .filter(user::Column::Name.eq(name.clone()))
            .order_by_asc(post::Column::CreatedAt)
            .all(&db)
            .await?;
        Ok(models
            .into_iter()
            .filter_map(|m| {
                PostBuilder::default()
                    .user_id(m.user_id)
                    .name(name.to_string())
                    .content(PostContent::new(m.content))
                    .build()
                    .ok()
            })
            .collect())
    }

    async fn list(&self) -> Result<Vec<Post>> {
        let db = connection().await?;
        let models = PostEntity::find()
            .column_as(user::Column::Name, "name")
            .join(JoinType::InnerJoin, user::Relation::Post.def())
            .into_model::<PostWithUser>()
            .all(&db)
            .await?;
        Ok(models
            .into_iter()
            .filter_map(|m| {
                PostBuilder::default()
                    .user_id(m.user_id)
                    .name(m.name)
                    .content(PostContent::new(m.content))
                    .build()
                    .ok()
            })
            .collect())
    }
}
