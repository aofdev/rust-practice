use actix_cors::Cors;
use actix_web::{http, middleware, App, HttpServer};
use config::{Config, IConfig};
use db::DB;
use std::default::Default;
use std::{env, io};

mod api;
mod config;
mod constants;
mod db;
mod dto;
mod error;
mod models;
mod services;

type Result<T> = std::result::Result<T, error::Error>;

#[actix_web::main]
async fn main() -> io::Result<()> {
    let config = Config {};
    env::set_var("RUST_LOG", "actix_web=debug");
    env_logger::init();

    let app_host = config.get_config_with_key("APP_HOST");
    let app_port = config.get_config_with_key("APP_PORT");
    let db_uri = config.get_config_with_key("DATABASE_URI");
    let db_name = config.get_config_with_key("DATABASE_NAME");
    let collection = config.get_config_with_key("COLLECTION");
    let app_url = format!("{}:{}", &app_host, &app_port);
    let db = match DB::init(db_uri, db_name, collection).await {
        Ok(db) => db,
        Err(err) => {
            println!("Failed to connect to database: {}", err);
            std::process::exit(1);
        }
    };

    HttpServer::new(move || {
        App::new()
            .wrap(
                Cors::default() // allowed_origin return access-control-allow-origin: * by default
                    .allowed_origin("http://127.0.0.1:3000")
                    .allowed_origin("http://localhost:3000")
                    .send_wildcard()
                    .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
                    .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
                    .allowed_header(http::header::CONTENT_TYPE)
                    .max_age(3600),
            )
            .wrap(middleware::Logger::default())
            .data(db.clone())
            .configure(api::config_services)
    })
    .bind(&app_url)?
    .run()
    .await
}

// #[cfg(test)]
// mod tests {
//     use actix_web::web::Bytes;
//     use actix_web::{test, web, App};

//     use crate::greet;

//     #[actix_rt::test]
//     async fn test_startup_ok() {
//         let mut app = test::init_service(App::new().route("/", web::get().to(greet))).await;
//         let req = test::TestRequest::with_header("content-type", "text/plain").to_request();
//         let result = test::read_response(&mut app, req).await;
//         assert_eq!(result, Bytes::from_static(b"Hello World!"));
//     }
// }
