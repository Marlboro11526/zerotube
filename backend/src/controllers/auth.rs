use crate::db::users;
use crate::messages::{
    auth::{LoginRequest, RegisterRequest, UserResponse},
    error::ErrorResponse,
};
use crate::models::user::User;
use crate::services::passwords;
use actix_session::Session;
use actix_web::{
    web::{Data, Json, Path},
    Error, HttpResponse,
};
use diesel::{r2d2::ConnectionManager, SqliteConnection};

type Pool = r2d2::Pool<ConnectionManager<SqliteConnection>>;

pub fn start(session: Session) -> HttpResponse {
    let username = session.get::<String>("username").unwrap_or(None);
    log::info!("Current user: {:?}", username);

    HttpResponse::Ok().json(UserResponse { username })
}

pub fn login(
    session: Session,
    request: Json<LoginRequest>,
    pool: Data<Pool>,
) -> Result<HttpResponse, Error> {
    let connection = pool.get().unwrap();

    let user = users::get_user_with_username(&request.username, &connection)?
        .ok_or(ErrorResponse::NotFound)?;

    if !passwords::compare_password(&request.password, &user.password)? {
        return Err(Error::from(ErrorResponse::NotFound));
    }

    session.set("username", &request.username)?;
    session.renew();
    log::info!("Logged in user '{:?}'", session.get::<String>("username"));

    Ok(HttpResponse::Ok().json(UserResponse {
        username: Some(request.username.clone()),
    }))
}

pub fn logout(session: Session) -> HttpResponse {
    session.purge();
    
    log::info!(
        "Logged out, user is now '{:?}'",
        session.get::<String>("username")
    );

    HttpResponse::Ok().finish()
}

pub fn register(pool: Data<Pool>, request: Json<RegisterRequest>) -> Result<HttpResponse, Error> {
    let connection = pool.get().unwrap();
    let user = users::get_user_with_email(&request.email, &connection)?;

    if user.is_some() {
        return Err(Error::from(ErrorResponse::BadRequest(
            "User already exists".into(),
        )));
    }

    let user = User::from(request.into_inner());
    users::create_user_and_confirmation_email(user, &connection)?;

    Ok(HttpResponse::Ok().finish())
}

pub fn register_confirm(
    confirmation_id: Path<String>,
    pool: Data<Pool>,
) -> Result<HttpResponse, Error> {
    let connection = pool.get().unwrap();
    users::confirm_registration(confirmation_id.clone(), &connection)?;

    Ok(HttpResponse::Ok().finish())
}
