use actix_web::{Result, web};
use serde::{Deserialize, Serialize};
use crate::errors::errors::ErrorResponse;
use crate::handlers::Response;

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
    nickname: String,
    password: String,
}

#[derive(Debug, Serialize)]
struct CreateUserResponse {
    user_id: String,
}


pub(self) async fn create_user_handler(user: web::Json<CreateUserRequest>) -> Result<Response<CreateUserResponse>, ErrorResponse> {
    let cur = CreateUserResponse { user_id: format!("Welcome {}, username {}, password {}!", user.nickname, user.username, user.password) };
    let resp = Response::<CreateUserResponse>::ok(cur);
    Ok(resp)
}

pub(self) async fn get_user_handler(path: web::Path<(u32, String)>) -> Result<String> {
    let (id, username) = path.into_inner();
    Ok(format!("Welcome {}, user_id {}!", username, id))
}