#[macro_use]
extern crate diesel;

use crate::middleware::Auth;
use actix_cors::Cors;
use actix_redis::RedisSession;
use actix_web::{http::header, middleware::Logger, web, App, HttpResponse, HttpServer};
use diesel::{r2d2::ConnectionManager, SqliteConnection};
use r2d2::Pool;
use std::{env, io};

mod auth;
mod db;
mod messages;
mod middleware;
mod models;
mod rooms;
mod util;
mod ws;

fn secret() -> HttpResponse {
    println!("POSTING SECRET MESSAGE");

    HttpResponse::Ok().json("SECRET MESSAGE")
}

fn main() -> io::Result<()> {
    dotenv::dotenv().ok();

    std::env::set_var(
        "RUST_LOG",
        "error,warn,info,actix_redis=info,actix_server=info,actix_web=info",
    );

    env_logger::init();
    let db_url = env::var("DB_URL").expect("Missing DB_URL in env");
    let manager = ConnectionManager::<SqliteConnection>::new(db_url);
    let pool = Pool::builder()
        .build(manager)
        .expect("Failed to create connection pool");

    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .wrap(RedisSession::new("localhost:6379", &[0; 32]).cookie_secure(false))
            .wrap(Logger::default())
            .wrap(
                Cors::new()
                    .allowed_origin("http://localhost:3000")
                    .allowed_methods(vec!["GET", "POST", "OPTIONS"])
                    .allowed_headers(vec![
                        header::AUTHORIZATION,
                        header::ACCEPT,
                        header::CONTENT_TYPE,
                    ])
                    .max_age(10)
                    .supports_credentials(),
            )
            .service(
                web::scope("/auth")
                    .route("/start", web::get().to(auth::start))
                    .route("/login", web::post().to(auth::login))
                    .route("/logout", web::post().to(auth::logout))
                    .route("/register", web::post().to(auth::register))
                    .route(
                        "/register/{confirmation_id}",
                        web::get().to(auth::register_confirm),
                    ),
            )
            .service(
                web::scope("/room")
                    .route("/create", web::post().to(rooms::create))
                    .route("/get", web::get().to(rooms::get_all))
                    .wrap(Auth),
            )
            .service(
                web::scope("/web")
                    .route("/secret", web::get().to(secret))
                    .wrap(Auth),
            )
            .service(
                web::scope("/ws")
                    .route("/time", web::get().to(ws::time::route))
                    .wrap(Auth),
            )
    })
    .bind("localhost:8081")?
    .run()
}
