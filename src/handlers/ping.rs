use actix_web::web;
use web::Data;
use crate::errors::errors::{ErrorResponse};
use crate::handlers::Response;
use crate::State;


pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/ping")
            .route(web::get().to(ping_handler))
    );
}


pub(crate) async fn ping_handler(state: Data<State<'_>>) -> Result<Response<String>, ErrorResponse> {
    if let Ok(row) = state.repo.ping.ping().await {
        Ok(Response::ok(format!("pong: {}", row)))
    } else {
        Err(ErrorResponse::INTERNAL_ERROR)
    }
}