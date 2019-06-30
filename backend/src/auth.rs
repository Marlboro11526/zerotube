use crate::messages::{ErrorResponse, LoginRequest, RegisterRequest, UserSession};
use actix_session::Session;
use actix_web::{web::Json, HttpResponse};

pub fn login(session: Session, request: Json<LoginRequest>) -> HttpResponse {
    if request.username != "user1" && request.password != "secret" {
        let error = ErrorResponse {
            error: "User not found".to_string(),
        };

        log::info!("No user found! Returning: {:?}", error);

        return HttpResponse::NotFound().json(error);
    }

    let user_session = UserSession {
        token: Some("foo".to_string()),
        username: Some("user1".to_string()),
    };

    let _ = session.set("user", user_session.clone());
    log::info!("user session {:?}", session.get::<UserSession>("user"));

    HttpResponse::Ok().json(user_session)
}

pub fn logout(session: Session) -> HttpResponse {
    session.remove("user");

    log::info!("user {:?}", session.get::<UserSession>("user"));

    let user_session = UserSession {
        token: None,
        username: None,
    };

    HttpResponse::Ok().json(user_session)
}

pub fn register(session: Session, request: Json<RegisterRequest>) {
    log::info!("registering: {} {}", request.0.username, request.0.email);

}
