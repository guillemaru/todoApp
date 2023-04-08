#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;

use actix_web::{web, App, HttpResponse, HttpServer, middleware};
use http::Method;
use dotenv::dotenv;
use listenfd::ListenFd;
use std::env;

mod db;
mod notes;
mod error_handler;
mod schema;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok(); //To manage environment variables
    db::init();
    let mut listenfd = ListenFd::from_env(); //Restarts the server when changes is detected in files
    let mut server = HttpServer::new(|| {
        App::new()
            .configure(notes::init_routes)
            .wrap(middleware::DefaultHeaders::new().header("Access-Control-Allow-Origin", "*"))
            .service(
                web::resource("/notes")
                    .route(web::post().to(|| HttpResponse::Ok()))
                    .route(
                        web::method(Method::OPTIONS).to(|| {
                            HttpResponse::Ok()
                            .header("Access-Control-Allow-Methods", "GET, POST, OPTIONS")
                            .header("Access-Control-Allow-Headers", "content-type")
                            .header("Access-Control-Allow-Origin", "*")
                            .finish()
                        })
                    )
            )
            .service(
                web::resource("/notes/*")
                    .route(web::put().to(|| HttpResponse::Ok()))
                    .route(web::delete().to(|| HttpResponse::Ok()))
                    .route(
                        web::method(Method::OPTIONS).to(|| {
                            HttpResponse::Ok()
                            .header("Access-Control-Allow-Methods", "PUT, DELETE, OPTIONS")
                            .header("Access-Control-Allow-Headers", "content-type")
                            .header("Access-Control-Allow-Origin", "*")
                            .finish()
                        })
                    )
            )
        }
    );
    server = match listenfd.take_tcp_listener(0)? {
        Some(listener) => server.listen(listener)?,
        None => {
            let host = env::var("HOST").expect("Please set host in .env");
            let port = env::var("PORT").expect("Please set port in .env");
            server.bind(format!("{}:{}", host, port))?
        }
    };
    server.run().await
}