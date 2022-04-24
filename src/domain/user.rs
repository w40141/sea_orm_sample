use derive_new::new;
use getset::Getters;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, new, Getters, Clone)]
#[getset(get = "pub")]
pub struct User {
    id: Option<i32>,
    name: String,
    email: String,
}
