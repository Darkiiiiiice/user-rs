use actix_web::web;
use crate::errors::errors::{ErrorResponse};
use crate::handlers::Response;


pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/ping")
            .route(web::get().to(ping_handler))
    );
}



pub(crate) async fn ping_handler() -> Result<Response<&'static str>, ErrorResponse> {
    Ok( Response::ok("pong"))
}