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
    async fn register_service(&self, domain: &User) -> Result<Option<User>>;
}

#[derive(new)]
struct UserSurface {
    service: Box<Arc<dyn IUserService + Sync + Send>>,
}

impl UserSurface {
    async fn register(&self, domain: &User) -> Result<Option<User>> {
        Ok(self.service.register_service(domain).await?)
    }
}

#[get("/user/register")]
pub async fn register_user() -> HttpResponse {
    HttpResponse::Ok().content_type("text/html").body(
        r#"
            <title>Register User</title>
            <form action="/registered" method="post">
                <input type="text" name="Name"/>
                <input type="text" name="Email"/>
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
        Ok(user) => match user {
            Some(u) => {
                let response = format!(
                    "Registered. ID = {:?}, Name = {}, Email = {}.\n",
                    u,
                    u.name(),
                    u.email()
                );
                HttpResponse::Ok().content_type("text/html").body(response)
            }
            None => HttpResponse::InternalServerError().body(format!("error",)),
        },
        Err(e) => HttpResponse::InternalServerError().body(format!("{e}")),
    }
}
