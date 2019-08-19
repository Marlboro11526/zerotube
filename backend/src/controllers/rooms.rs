use crate::db::rooms;
use crate::messages::{
    error::ErrorResponse,
    room::{RoomCreateRequest, RoomGetAllResponse},
};
use crate::models::room::Room;
use actix_web::{
    web::{Data, Json, Path},
    Error, HttpResponse,
};
use diesel::{r2d2::ConnectionManager, SqliteConnection};

type Pool = r2d2::Pool<ConnectionManager<SqliteConnection>>;

pub fn create(pool: Data<Pool>, request: Json<RoomCreateRequest>) -> Result<HttpResponse, Error> {
    let connection = pool.get().unwrap();

    if check_if_reserved_name(&request.name) {
        return Err(Error::from(ErrorResponse::BadRequest(
            "Name is reserved".into(),
        )));
    }

    if let Some(room) = rooms::get_room_with_name_or_url(&request.name, &request.url, &connection)?
    {
        if room.name == request.name {
            return Err(Error::from(ErrorResponse::BadRequest(
                "Name is in use".into(),
            )));
        } else if room.url == request.url {
            return Err(Error::from(ErrorResponse::BadRequest(
                "URL is in use".into(),
            )));
        } else {
            // should never reach this branch
            return Err(Error::from(ErrorResponse::InternalServerError));
        }
    }

    let room = Room::from(request.into_inner());
    rooms::create_room(room, &connection)?;

    Ok(HttpResponse::Ok().finish())
}

pub fn get_all(pool: Data<Pool>) -> Result<HttpResponse, Error> {
    let connection = pool.get().unwrap();
    let rooms = rooms::get_all_public(&connection)?;

    Ok(HttpResponse::Ok().json(RoomGetAllResponse { rooms }))
}

pub fn get(url: Path<String>, pool: Data<Pool>) -> Result<HttpResponse, Error> {
    let connection = pool.get().unwrap();
    let result = rooms::get_room_with_url(&url, &connection)?;

    Ok(HttpResponse::Ok().json(result))
}

fn check_if_reserved_name(name: &str) -> bool {
    const RESERVED_NAMES: [&str; 2] = ["confirm", "rooms"];

    RESERVED_NAMES.contains(&name)
}
