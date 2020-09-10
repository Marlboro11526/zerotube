#[macro_use]
extern crate diesel;

use crate::controllers::*;
use crate::middleware::Auth;
use actix_cors::Cors;
use actix_redis::RedisSession;
use actix_web::{http::header, middleware::Logger, web, App, HttpResponse, HttpServer};
use diesel::{r2d2::ConnectionManager, SqliteConnection};
use r2d2::Pool;
use rustls::{internal::pemfile, NoClientAuth, ServerConfig};
use std::{env, error::Error, fs::File, io::BufReader};

mod controllers;
mod db;
mod messages;
mod middleware;
mod models;
mod room_server;
mod services;
mod ws;

fn secret() -> HttpResponse {
    println!("POSTING SECRET MESSAGE");

    HttpResponse::Ok().json("SECRET MESSAGE")
}

fn main() -> Result<(), Box<dyn Error>> {
    dotenv::dotenv().ok();

    std::env::set_var(
        "RUST_LOG",
        "error,warn,info,actix_redis=info,actix_server=trace,actix_web=trace,rustls=info,actix_connect=trace,actix_https=trace,trust_dns_resolver=debug",
    );

    env_logger::init();
    let db_url = env::var("DB_URL").expect("Missing DB_URL in env");
    let manager = ConnectionManager::<SqliteConnection>::new(db_url);

    let pool = Pool::builder()
        .build(manager)
        .expect("Failed to create connection pool");

    let mut rustls_config = ServerConfig::new(NoClientAuth::new());
    let cert_file =
        &mut BufReader::new(File::open("cert.pem").map_err(|_| "Error getting cert.pem")?);
    let key_file = &mut BufReader::new(File::open("key.pem").map_err(|_| "Error getting key.pem")?);
    let cert_chain = pemfile::certs(cert_file).map_err(|_| "Error extracting certificates")?;
    let mut keys = pemfile::pkcs8_private_keys(key_file).map_err(|_| "Error extracting keys")?;
    rustls_config.set_single_cert(cert_chain, keys.remove(0))?;

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
                    .service(
                        web::scope("/media")
                            .route("/get/{room_id}", web::get().to(media::get_media_for_room))
                            .route("/add/{room_id}", web::post().to(media::add_media_to_room))
                            .route(
                                "/remove/{room_id}",
                                web::delete().to(media::remove_media_from_room),
                            ),
                    )
                    .wrap(Auth),
            )
            .service(
                web::scope("/rooms")
                    .route("/create", web::post().to(rooms::create))
                    .route("/get", web::get().to(rooms::get_all))
                    .route("/get/{room_url}", web::get().to(rooms::get))
                    .wrap(Auth),
            )
            .service(
                web::scope("/web")
                    .route("/secret", web::get().to(secret))
                    .wrap(Auth),
            )
            .service(
                web::scope("/ws")
                    .route(
                        "/session/{room_url}",
                        web::get().to(ws::room_session::route),
                    )
                    .route("/time", web::get().to(ws::time::route))
                    .wrap(Auth),
            )
    })
    .bind_rustls("localhost:8443", rustls_config)?
    .run()?;

    Ok(())
}
