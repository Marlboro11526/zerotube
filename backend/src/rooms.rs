use crate::db::rooms;
use crate::messages::{error::ErrorResponse, room::{
    RoomCreateRequest, RoomGetAllResponse,
    }};
use actix_web::{
    web::{Data, Json},
    HttpResponse, ResponseError,
};
use diesel::{r2d2::ConnectionManager, SqliteConnection};

type Connection = r2d2::PooledConnection<ConnectionManager<SqliteConnection>>;
type Pool = r2d2::Pool<ConnectionManager<SqliteConnection>>;

pub fn create(pool: Data<Pool>, request: Json<RoomCreateRequest>) -> HttpResponse {
    let connection = pool.get().unwrap();

    if check_if_reserved_name(&request.name) {
        return ErrorResponse::BadRequest("Name is reserved".into()).error_response();
    }

    if check_if_used_name(&request.name, &connection) {
        return ErrorResponse::BadRequest("Name is in use".into()).error_response();
    }

    let room = request.into_inner().into();
    let result = rooms::create_room(room, &connection);

    if result.is_err() {
        return ErrorResponse::InternalServerError.error_response();
    }

    HttpResponse::Ok().finish()
}

pub fn get_all(pool: Data<Pool>) -> HttpResponse {
    let connection = pool.get().unwrap();

    let result = rooms::get_all_public(&connection);

    if result.is_err() {
        return ErrorResponse::InternalServerError.error_response();
    }

    HttpResponse::Ok().json(RoomGetAllResponse {
        rooms: result.unwrap()
    })
}

fn check_if_reserved_name(name: &String) -> bool {
    const RESERVED_NAMES: [&str; 1] = ["confirm"];

    RESERVED_NAMES.contains(&name.as_str())
}

fn check_if_used_name(name: &String, connection: &Connection) -> bool {
    rooms::get_room_with_name(name, connection).is_err()
}
