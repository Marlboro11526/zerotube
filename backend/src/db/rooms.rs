use crate::db::entities::room::Room as DbRoom;
use crate::messages::error::ErrorResponse;
use crate::models::room::Room;
use diesel::{prelude::*, r2d2::ConnectionManager};
use r2d2::PooledConnection;

type Connection = PooledConnection<ConnectionManager<SqliteConnection>>;

pub fn get_room_with_url(
    url_input: &str,
    connection: &Connection,
) -> Result<Option<Room>, ErrorResponse> {
    use crate::db::schema::rooms::dsl::*;

    rooms
        .filter(url.eq(&url_input))
        .load::<DbRoom>(connection)
        .map(|mut result| Ok(result.pop().map(Room::from)))?
}

pub fn get_room_with_name_or_url(
    name_input: &str,
    url_input: &str,
    connection: &Connection,
) -> Result<Option<Room>, ErrorResponse> {
    use crate::db::schema::rooms::dsl::*;

    rooms
        .filter(name.eq(name_input))
        .or_filter(url.eq(url_input))
        .load::<DbRoom>(connection)
        .map(|mut result| Ok(result.pop().map(Room::from)))?
}

pub fn create_room(room: Room, connection: &Connection) -> Result<(), ErrorResponse> {
    use crate::db::schema::rooms::dsl::*;

    let room = DbRoom::new(room);

    diesel::insert_into(rooms)
        .values(room.clone())
        .execute(connection)
        .map(|_| Ok(()))?
}

pub fn get_all_public(connection: &Connection) -> Result<Vec<Room>, ErrorResponse> {
    use crate::db::schema::rooms::dsl::*;

    rooms
        .filter(public.eq(true))
        .load::<DbRoom>(connection)
        .map(|out| Ok(out.into_iter().map(Room::from).collect::<Vec<Room>>()))?
}

pub(crate) fn get_room_with_url_internal(
    url_input: &str,
    connection: &Connection,
) -> Result<Option<DbRoom>, ErrorResponse> {
    use crate::db::schema::rooms::dsl::*;

    rooms
        .filter(url.eq(&url_input))
        .load::<DbRoom>(connection)
        .map(|mut result| Ok(result.pop()))?
}
