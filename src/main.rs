mod handlers;
mod errors;

use actix_web::{web, App, HttpServer};
use actix_web::middleware::Logger;


#[derive(Debug, Default)]
struct State {
    app_name: String,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    std::env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();

    let state = web::Data::new(
        State {
            app_name: String::from("user-rs"),
        }
    );

    HttpServer::new(move || {
        let logger = Logger::default();

        let app = App::new()
            .app_data(state.clone())
            .wrap(logger)
            .configure(web_config);

        return app;
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}

fn web_config(cfg: &mut web::ServiceConfig) {
    let scope = web::scope("/api/v1");
    cfg.service(
        scope
            .configure(handlers::ping::init)
            .configure(handlers::users::init)
    );
}