use crate::auth::Auth;
use actix_cors::Cors;
use actix_session::{CookieSession, Session};
use actix_web::{
    http::header,
    middleware::Logger,
    web::{self, Json},
    App, Error, HttpResponse, HttpServer,
};
use diesel::{r2d2::ConnectionManager, SqliteConnection};
use futures::Future;
use r2d2::Pool;
use serde::{Deserialize, Serialize};
use std::{env, io};

mod auth;

#[derive(Debug, Deserialize, Serialize)]
pub struct UserSession {
    token: Option<String>,
    user: Option<String>,
}

fn index(session: Session) -> HttpResponse {
    let user = session.get::<UserSession>("user").unwrap_or(None);

    println!("Current user: {:?}", user);
    HttpResponse::Ok().json(user)
}

fn invalid() -> HttpResponse {
    println!("POSTING NOPE");

    HttpResponse::Ok().json("NOPE")
}

fn secret() -> HttpResponse {
    println!("POSTING SECRET MESSAGE");

    HttpResponse::Ok().json("SECRET MESSAGE")
}

fn login(session: Session) -> HttpResponse {
    let user = UserSession {
        token: None,
        user: Some("user1".to_string()),
    };
    let _ = session.set("user", user);

    log::info!("user {:?}", session.get::<UserSession>("user"));

    HttpResponse::Found().header("location", "/").finish()
}

fn logout(session: Session) -> HttpResponse {
    session.remove("user");

    log::info!("user {:?}", session.get::<UserSession>("user"));

    HttpResponse::Found().header("location", "/").finish()
}

fn main() -> io::Result<()> {
    dotenv::dotenv().ok();

    std::env::set_var(
        "RUST_LOG",
        "actix_test=debug,actix_web=info,actix_server=info",
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
            .wrap(CookieSession::signed(&[0; 32]).secure(false))
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
                    .route("/login", web::get().to(login))
                    .route("/logout", web::get().to(logout))
                    .route("/invalid", web::get().to(invalid)),
            )
            .service(
                web::scope("/web")
                    .route("/secret", web::get().to(secret))
                    .wrap(Auth),
            )
            .service(
                web::scope("/ws")
                    .route("/test", web::get().to(index))
                    .wrap(Auth),
            )
            .route("/", web::get().to(index))
    })
    .bind("localhost:8081")?
    .run()
}
