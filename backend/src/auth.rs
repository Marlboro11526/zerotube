use crate::db::users;
use crate::messages::{
    auth::{LoginRequest, RegisterRequest, UserResponse},
    error::ErrorResponse,
};
use crate::util;
use actix_session::Session;
use actix_web::{
    web::{Data, Json, Path},
    HttpResponse, ResponseError,
};
use diesel::{r2d2::ConnectionManager, SqliteConnection};

type Pool = r2d2::Pool<ConnectionManager<SqliteConnection>>;

pub fn start(session: Session) -> HttpResponse {
    let username = session.get::<String>("username").unwrap_or(None);

    log::info!("Current user: {:?}", username);

    HttpResponse::Ok().json(UserResponse {
        username
    })
}

pub fn login(session: Session, request: Json<LoginRequest>, pool: Data<Pool>) -> HttpResponse {
    let connection = pool.get().unwrap();

    let user = users::get_user_with_username(&request.username, &connection);

    if user.is_err() {
        return HttpResponse::NotFound().finish();
    }

    let is_match = util::compare_password(&request.password, &user.unwrap().password);

    match is_match {
        Err(_) => return HttpResponse::NotFound().finish(),
        Ok(false) => return HttpResponse::NotFound().finish(),
        Ok(true) => (),
    }

    let result = session.set("username", &request.username);

    if result.is_err() {
        return ErrorResponse::InternalServerError.error_response();
    };

    session.renew();

    log::info!("user session {:?}", session.get::<String>("username"));

    HttpResponse::Ok().json(UserResponse {
        username: Some(request.username.clone()),
    })
}

pub fn logout(session: Session) -> HttpResponse {
    session.purge();

    log::info!("username {:?}", session.get::<String>("username"));

    HttpResponse::Ok().finish()
}

pub fn register(pool: Data<Pool>, request: Json<RegisterRequest>) -> HttpResponse {
    println!("TEST");
    log::info!("registering: {} {}", request.0.username, request.0.email);

    let connection = pool.get().unwrap();
    let user = users::get_user_with_email(&request.email, &connection);

    if user.is_err() {
        log::error!("Error on getting user with email {}", request.email);

        return user.unwrap_err().error_response();
    }

    if user.unwrap().is_some() {
        log::warn!(
            "User attempted to register with existing email {}",
            request.email
        );

        return ErrorResponse::BadRequest("User already exists".into()).error_response();
    }

    let user = request.into_inner().into();
    let result = users::create_user_and_confirmation_email(user, &connection);

    if result.is_err() {
        log::error!("Error on creating user and confirmation email");

        return result.unwrap_err().error_response();
    }

    HttpResponse::Ok().finish()
}

pub fn register_confirm(confirmation_id: Path<String>, pool: Data<Pool>) -> HttpResponse {
    let connection = pool.get().unwrap();

    let result = users::confirm_registration(confirmation_id.clone(), &connection);

    if result.is_err() {
        log::error!(
            "Error on confirming registration for ID {}",
            confirmation_id.into_inner()
        );

        return result.unwrap_err().error_response();
    }

    HttpResponse::Ok().finish()
}
