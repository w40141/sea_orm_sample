use anyhow::Result;
use async_trait::async_trait;
use derive_builder::Builder;
use derive_new::new;
use getset::Getters;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[async_trait]
pub trait IPostRepository {
    async fn insert(&self, user: &Post) -> Result<u64>;
    async fn find_by_user_id(&self, user_id: &u64) -> Result<Vec<Post>>;
    async fn find_by_user_name(&self, name: &String) -> Result<Vec<Post>>;
    async fn list(&self) -> Result<Vec<Post>>;
}

#[derive(Debug, Serialize, Deserialize, new, Getters, Clone, Builder, PartialEq)]
#[getset(get = "pub")]
#[builder(setter(into))]
pub struct Post {
    user_id: u64,
    name: String,
    content: PostContent,
}

#[derive(Debug, Serialize, Deserialize, new, Getters, Clone, Validate, PartialEq)]
#[getset(get = "pub")]
pub struct PostContent {
    #[validate(length(max = 200))]
    content: String,
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn content_test() {
        {
            let content = PostContent::new("test".to_string());
            assert_eq!(content.validate().is_ok(), true);
        }
        {
            let content = PostContent::new(
                "
                Lorem ipsum dolor sit amet, consectetur adipiscing elit.
                Curabitur in facilisis nisl. Curabitur tempus nisl sit amet
                quam consequat pulvinar. Fusce rutrum aliquet dictum.
                Suspendisse tempus finibus enim a pharetra.
                Aliquam cursus metus eget lacus ultricies, vitae viverra libero rhoncus.
                Nullam purus lectus, volutpat quis ex nec, mollis maximus tortor.
                Curabitur commodo vulputate risus quis aliquam.
                Integer pulvinar sodales sollicitudin. Aenean a neque nibh."
                    .to_string(),
            );
            assert_eq!(content.validate().is_err(), true);
        }
    }

    #[test]
    fn post_test() {
        {
            let post: Result<Post, PostBuilderError> = PostBuilder::default()
                .user_id(1 as u64)
                .name("Taro".to_string())
                .content(PostContent::new("test".to_string()))
                .build();
            assert_eq!(&post.is_ok(), &true);
            let post = post.unwrap();
            assert_eq!(
                post,
                Post {
                    user_id: 1,
                    name: "Taro".to_string(),
                    content: PostContent::new("test".to_string())
                }
            );
        }
        {
            let post: Result<Post, PostBuilderError> =
                PostBuilder::default().user_id(1 as u64).build();
            assert_eq!(&post.is_err(), &true);
        }
    }
}
