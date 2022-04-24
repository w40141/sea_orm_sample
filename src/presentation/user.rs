use actix_web::{get, web, HttpResponse};
use anyhow::Result;
use async_trait::async_trait;
use derive_new::new;
use std::sync::Arc;

use crate::application::user::UserService;
use crate::domain::user::User;
use crate::infrastructure::user::UserRepository;

#[async_trait]
pub trait IUserService {
    async fn register(&self) -> Result<Option<User>>;
}

#[derive(new)]
struct UserSurface {
    service: Arc<dyn UserService + Sync + Send>,
}

impl UserSurface {
    async fn register(&self) -> Result<Option<User>> {
        Ok(self.service.register().await?)
    }
}

#[get("/user/register")]
async fn register_user() -> HttpResponse {
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

async fn registered(form: web::Form<User>) -> Result<HttpResponse> {
    let user = User::new(None, form.name().to_owned(), form.email().to_owned());
    let repository = Arc::new(&UserRepository::new(user));
    let service = Arc::new(&UserService::new(repository));
    let presentation = Arc::new(&UserSurface::new(service));
    let user_registered = presentation.register().await?;
    let response = match user_registered {
        Some(u) => {
            format!(
                "Registered. ID = {:?}, Name = {}, Email = {}.\n",
                u,
                u.name(),
                u.email()
            )
        }

        None => String::from("Error"),
    };
    Ok(HttpResponse::Ok().content_type("text/html").body(response))
}
