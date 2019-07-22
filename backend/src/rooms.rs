use crate::db::rooms;
use crate::messages::{error::ErrorResponse, room::{
    RoomCreateRequest, RoomGetAllResponse,
    }};
use crate::models::room::Room;
use actix_web::{
    web::{Data, Json, Path},
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

    if let Ok(room) = get_with_name_or_url(&request.name, &request.url, &connection) {
        if room.name == request.name {
            return ErrorResponse::BadRequest("Name is in use".into()).error_response();
        } else if room.url == request.url {
            return ErrorResponse::BadRequest("URL is in use".into()).error_response();
        } else {
            return ErrorResponse::InternalServerError.error_response();
        }
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

    log::info!("Retrieved all rooms: {:?}", result);

    HttpResponse::Ok().json(RoomGetAllResponse {
        rooms: result.unwrap()
    })
}

pub fn get(url: Path<String>, pool: Data<Pool>) -> HttpResponse {
    let connection = pool.get().unwrap();

    let result = rooms::get_room_with_url(url.clone(), &connection);

    if result.is_err() {
        return ErrorResponse::InternalServerError.error_response();
    }

    log::info!("Retrieved room: {:?}", result);

    HttpResponse::Ok().json(result.unwrap())
}

fn check_if_reserved_name(name: &str) -> bool {
    const RESERVED_NAMES: [&str; 2] = ["confirm", "rooms"];

    RESERVED_NAMES.contains(&name)
}

fn get_with_name_or_url(name: &str, url: &str, connection: &Connection) -> Result<Room, ErrorResponse> {
    rooms::get_room_with_name_and_url(name, url, connection)
}
