use anyhow::Result;
use async_trait::async_trait;
use derive_builder::Builder;
use derive_new::new;
use getset::Getters;
use serde::{Deserialize, Serialize};

use super::post::Post;

#[async_trait]
pub trait IFavoriteRepository {
    async fn register(&self, owner_id: &u64, post_id: &u64) -> Result<u64>;
    async fn find_by_user_id(&self, id: &u64) -> Result<Favorite>;
    async fn find_by_user_name(&self, name: &String) -> Result<Favorite>;
    // async fn list(&self) -> Result<Vec<Favorite>>;
}

#[derive(Debug, Serialize, Deserialize, new, Getters, Clone, Builder, PartialEq)]
#[getset(get = "pub")]
#[builder(setter(into))]
pub struct Favorite {
    owner_id: u64,
    posts: Vec<Post>,
}
