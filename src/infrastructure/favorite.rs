use anyhow::{anyhow, Result};
use derive_new::new;
use migration::JoinType;

use sea_orm::prelude::DateTimeUtc;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, EntityTrait, FromQueryResult, QueryFilter, QueryOrder,
    QuerySelect, RelationTrait, Set,
};

use super::configure::connection;

use crate::domain::favorite::{Favorite, FavoriteBuilder, IFavoriteRepository};
use crate::domain::post::{Post, PostBuilder, PostContent};
use crate::entity::favorite::{self, ActiveModel, Entity as FavoriteEntity};
use crate::entity::{post, user};

#[derive(new)]
pub struct FavoriteRepository {}

#[derive(FromQueryResult)]
struct FavoritePost {
    id: u64,
    user_id: u64,
    post_id: u64,
    post_user_id: u64,
    post_user_name: String,
    content: String,
    created_at: DateTimeUtc,
    updated_at: DateTimeUtc,
}

#[async_trait::async_trait]
impl IFavoriteRepository for FavoriteRepository {
    async fn register(&self, owner_id: &u64, post_id: &u64) -> Result<u64> {
        let db = connection().await?;
        let active_model = ActiveModel {
            user_id: Set(owner_id.clone()),
            post_id: Set(post_id.clone()),
            created_at: Set(chrono::Utc::now()),
            updated_at: Set(chrono::Utc::now()),
            ..Default::default()
        };
        let model = active_model.insert(&db).await?;
        Ok(model.id)
    }

    async fn find_by_user_id(&self, owner_id: &u64) -> Result<Favorite> {
        let db = connection().await?;
        let models = FavoriteEntity::find()
            .column_as(user::Column::Id, "post_user_id")
            .column_as(user::Column::Name, "post_user_name")
            .join(JoinType::InnerJoin, favorite::Relation::Post.def())
            .join(JoinType::InnerJoin, post::Relation::User.def())
            .filter(favorite::Column::UserId.eq(owner_id.clone()))
            .order_by_asc(favorite::Column::CreatedAt)
            .into_model::<FavoritePost>()
            .all(&db)
            .await?;
        let posts: Vec<Post> = models
            .into_iter()
            .filter_map(|m| {
                PostBuilder::default()
                    .user_id(m.post_user_id)
                    .name(m.post_user_name)
                    .content(PostContent::new(m.content))
                    .build()
                    .ok()
            })
            .collect();
        Ok(FavoriteBuilder::default()
            .owner_id(owner_id.clone())
            .posts(posts)
            .build()?)
    }

    async fn find_by_user_name(&self, name: &String) -> Result<Favorite> {
        let db = connection().await?;
        let model = user::Entity::find()
            .filter(user::Column::Name.eq(name.clone()))
            .one(&db)
            .await?;
        let user_id = match model {
            Some(m) => Ok(m.id),
            _ => Err(anyhow!("Not Found {name}")),
        }?;
        let models = FavoriteEntity::find()
            .column_as(user::Column::Id, "post_user_id")
            .column_as(user::Column::Name, "post_user_name")
            .join(JoinType::InnerJoin, favorite::Relation::Post.def())
            .join(JoinType::InnerJoin, post::Relation::User.def())
            .filter(favorite::Column::UserId.eq(user_id.clone()))
            .order_by_asc(favorite::Column::CreatedAt)
            .into_model::<FavoritePost>()
            .all(&db)
            .await?;
        let posts: Vec<Post> = models
            .into_iter()
            .filter_map(|m| {
                PostBuilder::default()
                    .user_id(m.post_user_id)
                    .name(m.post_user_name)
                    .content(PostContent::new(m.content))
                    .build()
                    .ok()
            })
            .collect();
        Ok(FavoriteBuilder::default()
            .owner_id(user_id.clone())
            .posts(posts)
            .build()?)
    }

    // async fn list(&self) -> Result<Vec<Favorite>> {
    //     let db = connection().await?;
    //     let models = FavoriteEntity::find()
    //         .column_as(user::Column::Name, "owner_id")
    //         .join(JoinType::InnerJoin, favorite::Relation::User.def())
    //         .column_as(user::Column::Id, "post_user_id")
    //         .column_as(user::Column::Name, "post_user_name")
    //         .join(JoinType::InnerJoin, favorite::Relation::Post.def())
    //         .join(JoinType::InnerJoin, post::Relation::User.def())
    //         .order_by_asc(favorite::Column::CreatedAt)
    //         .into_model::<FavoritePost>()
    //         .all(&db)
    //         .await?;
    //     let posts: Vec<Post> = models
    //         .into_iter()
    //         .filter_map(|m| {
    //             PostBuilder::default()
    //                 .user_id(m.post_user_id)
    //                 .name(m.post_user_name)
    //                 .content(PostContent::new(m.content))
    //                 .build()
    //                 .ok()
    //         })
    //         .collect();
    //     Ok(FavoriteBuilder::default()
    //         .owner_id(owner_id.clone())
    //         .posts(posts)
    //         .build()?)
    // }
}
