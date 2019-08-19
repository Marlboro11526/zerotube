use crate::db::{entities::room_media::RoomMedia as DbMedia, rooms};
use crate::messages::error::ErrorResponse;
use crate::models::media::Media;
use diesel::{prelude::*, r2d2::ConnectionManager, Connection as DieselConnection};
use r2d2::PooledConnection;

type Connection = PooledConnection<ConnectionManager<SqliteConnection>>;

pub fn get_media_for_room_with_url(
    room_url_input: &str,
    connection: &Connection,
) -> Result<Vec<Media>, ErrorResponse> {
    use crate::db::schema::room_media::dsl::*;

    connection.transaction(|| {
        let room_id_input = rooms::get_room_with_url_internal(room_url_input, connection)?
            .ok_or(ErrorResponse::NotFound)?
            .id;

        room_media
            .filter(room_id.eq(&room_id_input))
            .load::<DbMedia>(connection)
            .map(|out| Ok(out.into_iter().map(Media::from).collect::<Vec<Media>>()))?
    })
}

pub fn add_media_to_room_with_url(
    room_url_input: &str,
    media: Media,
    connection: &Connection,
) -> Result<(), ErrorResponse> {
    use crate::db::schema::room_media::dsl::*;

    connection.transaction(|| {
        let room_id_input = rooms::get_room_with_url_internal(room_url_input, connection)?
            .ok_or(ErrorResponse::NotFound)?
            .id;

        let media = DbMedia::new(media, &room_id_input)?;

        diesel::insert_into(room_media)
            .values(media.clone())
            .execute(connection)
            .map(|_| Ok(()))?
    })
}

pub fn remove_media_from_room_with_url(
    room_url_input: &str,
    media_url: &str,
    connection: &Connection,
) -> Result<(), ErrorResponse> {
    use crate::db::schema::room_media::dsl::*;

    connection.transaction(|| {
        let room_id_input = rooms::get_room_with_url_internal(room_url_input, connection)?
            .ok_or(ErrorResponse::NotFound)?
            .id;

        diesel::delete(room_media.filter(room_id.eq(&room_id_input).and(media_url.eq(media_url))))
            .execute(connection)
            .map(|_| Ok(()))?
    })
}
