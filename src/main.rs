#[macro_use]
extern crate diesel;

#[macro_use]
extern crate diesel_migrations;

use actix_web::{App, HttpServer};
use dotenv::dotenv;
use listenfd::ListenFd;
use std::env;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    db::init();

    let mut listenfd = ListenFd::from_env();
    let mut server = HttpServer::new(|| App::new().configure(users::init_routes));

    server = match listenfd.take_tcp_listener(0)? {
        Some(listener) => server.listen(listener)?,
        None => {
            let host: String = env::var("HOST").expect("Please set host in .env");
            let port: String = env::var("PORT").expect("Please set port in .env");
            server.bind(format!("{}:{}", host, port))?
        }
    };

    server.run().await
}
