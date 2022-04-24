use actix_web::{get, web, HttpResponse};
use anyhow::Result;
use std::sync::Arc;

use crate::domain::user::User;

#[async_trait::async_trait]
pub trait IUserService {
    async fn register(&self, user: &User) -> Result<Option<User>>;
}

#[derive()]
struct UserPresentation {
    service: Arc<dyn IUserService + Sync + Send>,
}

impl UserPresentation {
    async fn register(&self, user: &User) -> Result<Option<User>> {
        Ok(self.service.register(user).await?)
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
    let presentation = UserPresentation {
        service: UserService,
    };
    let user_registered = presentation
        .register(&User::new(
            None,
            form.name().to_owned(),
            form.email().to_owned(),
        ))
        .await?;
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
