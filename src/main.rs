mod handlers;
mod errors;
mod repos;
mod models;
mod middlewares;

use std::net::SocketAddr;
use std::sync::Arc;
use actix_web::{web, App, HttpServer};
use actix_web::middleware::Logger;
use env_logger::Env;
use crate::middlewares::authorization::Authorization;
use crate::repos::Repository;
use clap::{Args, Parser};


// const APP_NAME: &str = "user-rs";

#[derive(Parser, Debug)]
#[command(author = "darkiiiiiice")]
#[command(version = "0.0.1")]
#[command(version, about, long_about = None)]
struct CommonArgs {
    /// Http server listen host
    #[arg(short, long, default_value = "127.0.0.1:8880")]
    address: Option<SocketAddr>,

    /// Http server listen port
    #[arg(short = 'P', long = "port", default_value_t = 8888)]
    port: u16,

    /// Turn debugging information on
    #[arg(short, long, default_value_t = false)]
    debug: bool,

    #[command(flatten)]
    postgres: PostgresArgs,
}

#[derive(Debug, Args)]
#[command(args_conflicts_with_subcommands = true)]
#[command(flatten_help = true)]
struct PostgresArgs {
    #[arg(short, long)]
    host: String,
}

#[derive(Debug)]
struct State<'a> {
    repo: Arc<Repository<'a>>,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let args = CommonArgs::parse();
    println!("listen: {:?}", args.address);
    println!("debug mode: {:?}", args.debug);

    if args.debug {
        env_logger::init_from_env(Env::default().default_filter_or("debug"));
    } else {
        env_logger::init_from_env(Env::default().default_filter_or("info"));
    }

    let repo = Arc::new(Repository::new().await);
    let state = web::Data::new(
        State {
            repo: repo.clone(),
        }
    );

    let addr = args.address.expect("address not set");

    HttpServer::new(move || {
        let logger = Logger::new("<%a> '%r' %s %bB Auth=[%{Authorization}i] (exec: %Dms)");
        let auth = Authorization::new(repo.clone());

        let app = App::new()
            .app_data(state.clone())
            .wrap(logger)
            .wrap(auth)
            .configure(web_config);

        return app;
    })
        .bind(addr)?
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