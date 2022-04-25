use actix_web::{get, post, web, HttpResponse};
use anyhow::Result;
use async_trait::async_trait;
use derive_new::new;
use std::sync::Arc;

use crate::application::user::UserService;
use crate::domain::user::User;
use crate::infrastructure::user::UserRepository;

#[async_trait]
pub trait IUserService {
    async fn register_user_service(&self, domain: &User) -> Result<User>;
    async fn delete_user_service(&self, domain: &User) -> Result<User>;
    async fn change_user_name_service(&self, domain: &User) -> Result<User>;
    async fn change_user_email_service(&self, domain: &User) -> Result<User>;
}

#[derive(new)]
struct UserSurface {
    service: Box<Arc<dyn IUserService + Sync + Send>>,
}

impl UserSurface {
    async fn register(&self, domain: &User) -> Result<User> {
        Ok(self.service.register_user_service(domain).await?)
    }
}

#[get("/user/register")]
pub async fn register_user() -> HttpResponse {
    HttpResponse::Ok().content_type("text/html").body(
        r#"
            <title>Register User</title>
            <form action="/user/registered" method="post">
                <input type="text" name="name"/>
                <input type="text" name="email"/>
                <button type="submit">Register User</button>
            </form>
        "#,
    )
}

#[post("/user/registered")]
pub async fn registered(form: web::Form<User>) -> HttpResponse {
    let repository = Arc::new(UserRepository::new());
    let service = Arc::new(UserService::new(repository));
    let presentation = &UserSurface::new(Box::new(service));
    let user = User::new(None, form.name().to_owned(), form.email().to_owned());
    let result = presentation.register(&user).await;
    match result {
        Ok(user) => {
            let response = format!(
                "Registered. ID = {:?}, Name = {}, Email = {}.\n",
                user.id(),
                user.name(),
                user.email()
            );
            HttpResponse::Ok().content_type("text/html").body(response)
        }
        Err(e) => {
            log::error!("{e:?}");
            HttpResponse::InternalServerError().body(format!("Internal Server Error"))
        }
    }
}
