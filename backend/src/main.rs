use crate::messages::{ErrorResponse, UserSession};
use crate::middleware::Auth;
use actix_cors::Cors;
use actix_redis::RedisSession;
use actix_session::Session;
use actix_web::{
    http::header, middleware::Logger, web, App, HttpRequest, HttpResponse, HttpServer,
};
use diesel::{r2d2::ConnectionManager, SqliteConnection};
use r2d2::Pool;
use std::{env, io};

mod auth;
pub mod messages;
mod middleware;

fn index(session: Session) -> HttpResponse {
    let user = session.get::<UserSession>("user").unwrap_or(None);

    println!("Current user: {:?}", user);
    HttpResponse::Ok().json(user)
}

fn invalid() -> HttpResponse {
    println!("POSTING NOPE");

    HttpResponse::Ok().json("NOPE")
}

fn secret(request: HttpRequest) -> HttpResponse {
    if request.headers().get("authorization") == None {
        let error = ErrorResponse {
            error: "WHERE'S YR TOKEN??".to_string(),
        };

        return HttpResponse::NotFound().json(error);
    }
    println!("POSTING SECRET MESSAGE");

    HttpResponse::Ok().json("SECRET MESSAGE")
}

fn main() -> io::Result<()> {
    dotenv::dotenv().ok();

    std::env::set_var(
        "RUST_LOG",
        "error,warn,actix_redis=info,actix_server=info,actix_web=info",
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
                    .route("/login", web::post().to(auth::login))
                    .route("/logout", web::post().to(auth::logout))
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
