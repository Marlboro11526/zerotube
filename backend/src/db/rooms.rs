use crate::db::entities::room::Room as DbRoom;
use crate::messages::error::ErrorResponse;
use crate::models::room::Room;
use diesel::{prelude::*, r2d2::ConnectionManager};
use r2d2::PooledConnection;

type Connection = PooledConnection<ConnectionManager<SqliteConnection>>;

pub fn get_room_with_url(
    url_input: String,
    connection: &Connection,
) -> Result<Room, ErrorResponse> {
    use crate::db::schema::rooms::dsl::*;

    rooms
        .filter(url.eq(&url_input))
        .load::<DbRoom>(connection)
        .map_err(|err| {
            log::error!(
                "Failed to load room from DB.\nError: {}\nURL: {}",
                err,
                url_input,
            );

            ErrorResponse::BadRequest("Invalid URL".into())
        })
        .and_then(|mut result| {
            if let Some(room) = result.pop() {
                Ok(room.into())
            } else {
                Err(ErrorResponse::BadRequest("Room not found".into()))
            }
        })
}

pub fn get_room_with_name_and_url(
    name_input: &str,
    url_input: &str,
    connection: &Connection,
) -> Result<Room, ErrorResponse> {
    use crate::db::schema::rooms::dsl::*;

    rooms
        .filter(name.eq(name_input))
        .or_filter(url.eq(url_input))
        .load::<DbRoom>(connection)
        .map_err(|err| {
            log::error!(
                "Failed to load room from DB.\nError: {}\nName: {}\nURL: {}",
                err,
                name_input,
                url_input,
            );

            ErrorResponse::BadRequest("Invalid URL".into())
        })
        .and_then(|mut result| {
            if let Some(room) = result.pop() {
                Ok(room.into())
            } else {
                Err(ErrorResponse::BadRequest("Room not found".into()))
            }
        })
}

pub fn create_room(room: Room, connection: &Connection) -> Result<(), ErrorResponse> {
    use crate::db::schema::rooms;

    let room = DbRoom::new(room);

    diesel::insert_into(rooms::table)
        .values(room.clone())
        .execute(connection)
        .map(|_| ())
        .map_err(|err| {
            log::error!(
                "Failed to insert room into DB.\nError: {}\nRoom: {:?}",
                err,
                room
            );

            ErrorResponse::InternalServerError
        })
}

pub fn get_all_public(connection: &Connection) -> Result<Vec<Room>, ErrorResponse> {
    use crate::db::schema::rooms::dsl::*;

    rooms
        .filter(public.eq(true))
        .load::<DbRoom>(connection)
        .map(|out| {
            out.into_iter()
                .map(Room::from)
                .collect::<Vec<Room>>()
        })
        .map_err(|err| {
            log::error!("Failed to get all public rooms from DB.\nError: {}", err);

            ErrorResponse::InternalServerError
        })
}
