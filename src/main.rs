mod handlers;
mod errors;
mod repos;
mod models;

use std::sync::Arc;
use actix_web::{web, App, HttpServer};
use actix_web::middleware::Logger;
use env_logger::Env;
use crate::repos::Repository;


// const APP_NAME: &str = "user-rs";

#[derive(Debug)]
struct State<'a> {
    repo: Arc<Repository<'a>>,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    std::env::set_var("RUST_BACKTRACE", "1");
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    let repo = Repository::new().await;

    let state = web::Data::new(
        State {
            repo: Arc::new(repo),
        }
    );

    HttpServer::new(move || {
        let logger = Logger::new("<%a> '%r' %s %bB Auth=[%{Authorization}i] (exec: %Dms)");

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