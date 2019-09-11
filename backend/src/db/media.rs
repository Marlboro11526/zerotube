use crate::db::{entities::room_media::RoomMedia as DbMedia, rooms};
use crate::messages::error::ErrorResponse;
use crate::models::media::Media;
use diesel::{prelude::*, r2d2::ConnectionManager, Connection as DieselConnection};
use r2d2::PooledConnection;
use std::convert::TryFrom;

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

    let media_index = i32::try_from(media.index).map_err(|_| ErrorResponse::InternalServerError)?;

    connection.transaction(|| {
        let room_id_input = rooms::get_room_with_url_internal(room_url_input, connection)?
            .ok_or(ErrorResponse::NotFound)?
            .id;

        let media_for_index_increments = room_media.filter(
            room_id
                .eq(&room_id_input)
                .and(room_media_index.ge(media_index)),
        );

        diesel::update(media_for_index_increments)
            .set(room_media_index.eq(room_media_index + 1))
            .execute(connection)?;

        let media = DbMedia::new(media, &room_id_input)?;

        diesel::insert_into(room_media)
            .values(media)
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

pub fn get_latest_media_index_for_room_with_url(
    room_url_input: &str,
    connection: &Connection,
) -> Result<u32, ErrorResponse> {
    use crate::db::schema::room_media::dsl::*;

    connection.transaction(|| {
        let room_id_input = rooms::get_room_with_url_internal(room_url_input, connection)?
            .ok_or(ErrorResponse::NotFound)?
            .id;

        room_media
            .filter(room_id.eq(&room_id_input))
            .order(room_media_index.desc())
            .limit(1)
            .load::<DbMedia>(connection)
            .map(|mut out| {
                if let Some(media) = out.pop() {
                    Ok(u32::try_from(media.room_media_index)
                        .map_err(|_| ErrorResponse::InternalServerError)?)
                } else {
                    Ok(0)
                }
            })?
    })
}
