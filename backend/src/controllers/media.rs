use crate::db::media;
use crate::messages::media::{AddMediaRequest, GetAllMediaResponse, RemoveMediaRequest};
use crate::models::media::Media;
use actix_web::{
    web::{Data, Json, Path},
    Error, HttpResponse,
};
use diesel::{r2d2::ConnectionManager, SqliteConnection};

type Pool = r2d2::Pool<ConnectionManager<SqliteConnection>>;

pub fn get_media_for_room(room_url: Path<String>, pool: Data<Pool>) -> Result<HttpResponse, Error> {
    let connection = pool.get().unwrap();
    let media = media::get_media_for_room_with_url(&room_url, &connection)?;

    Ok(HttpResponse::Ok().json(GetAllMediaResponse { media }))
}

pub fn add_media_to_room(
    room_url: Path<String>,
    request: Json<AddMediaRequest>,
    pool: Data<Pool>,
) -> Result<HttpResponse, Error> {
    log::info!("In method...");
    let connection = pool.get().unwrap();
    log::info!("Got connection");
    let media = Media::new(&request.url)?;
    log::info!("Got media");
    media::add_media_to_room_with_url(&room_url, media, &connection)?;
    log::info!("Added media");

    Ok(HttpResponse::Ok().finish())
}

pub fn remove_media_from_room(
    room_url: Path<String>,
    request: Json<RemoveMediaRequest>,
    pool: Data<Pool>,
) -> Result<HttpResponse, Error> {
    let connection = pool.get().unwrap();
    media::remove_media_from_room_with_url(&room_url, &request.url, &connection)?;

    Ok(HttpResponse::Ok().finish())
}
