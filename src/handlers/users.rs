use actix_web::{Result, web};
use actix_web::web::Data;
use chrono::Utc;
use serde::{Deserialize, Serialize};
use crate::errors::errors::ErrorResponse;
use crate::handlers::Response;
use crate::models::user::User;
use crate::State;

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/user/{id}")
            .route(web::get().to(get_user_handler))
    )
        .service(
            web::resource("/user")
                .route(web::post().to(create_user_handler))
        );
}

#[derive(Debug, Deserialize)]
struct CreateUserRequest {
    username: String,
    email: String,
    phone: String,
}

#[derive(Debug, Serialize)]
struct CreateUserResponse {
    user_id: String,
}


pub(self) async fn create_user_handler(state: Data<State<'_>>, user: web::Json<CreateUserRequest>) -> Result<Response<CreateUserResponse>, ErrorResponse> {
    let mut input = User::new(&user.username, &user.email, &user.phone);
    input.created_at = Utc::now();

    let user_id = state.repo.users.create_common_user(input).await?;
    let resp = Response::<CreateUserResponse>::ok(
        CreateUserResponse { user_id: user_id.to_string() }
    );
    Ok(resp)
}

pub(self) async fn get_user_handler(path: web::Path<(u32, String)>) -> Result<String> {
    let (id, username) = path.into_inner();
    Ok(format!("Welcome {}, user_id {}!", username, id))
}