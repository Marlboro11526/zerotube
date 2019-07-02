use crate::db::users;
use crate::messages::{
    auth::{LoginRequest, RegisterRequest},
    error::ErrorResponse,
};
use crate::models::user::User;
use actix_session::Session;
use actix_web::{
    web::{self, Json},
    Error, HttpResponse, ResponseError,
};
use diesel::{r2d2::ConnectionManager, SqliteConnection};

type Pool = r2d2::Pool<ConnectionManager<SqliteConnection>>;

pub fn login(session: Session, request: Json<LoginRequest>) -> HttpResponse {
    if request.username != "user1" && request.password != "secret" {
        let error = ErrorResponse::BadRequest("User not found".to_string());

        log::info!("No user found! Returning: {:?}", error);

        return HttpResponse::NotFound().json(error);
    }

    let _ = session.set("username", "user1");
    log::info!("user session {:?}", session.get::<String>("username"));

    HttpResponse::Ok().json("user1")
}

pub fn logout(session: Session) -> HttpResponse {
    session.remove("user");

    log::info!("user {:?}", session.get::<String>("username"));

    HttpResponse::Ok().finish()
}

pub fn register(
    session: Session,
    pool: web::Data<Pool>,
    request: Json<RegisterRequest>,
) -> HttpResponse {
    println!("TEST");
    log::info!("registering: {} {}", request.0.username, request.0.email);

    let connection = pool.get().unwrap();
    let user = users::get_user_with_email(&request.email, &connection);

    if user.is_err() {
        return user.unwrap_err().error_response();
    }

    if let Some(user) = user.unwrap() {
        return ErrorResponse::BadRequest("User already exists".into()).error_response();
    }

    let user = User {
        email: request.email.clone(),
        username: request.username.clone(),
        password: request.password.clone(),
    };

    let result = users::create_user_and_confirmation_email(user, &connection);

    HttpResponse::Ok().finish()
}
