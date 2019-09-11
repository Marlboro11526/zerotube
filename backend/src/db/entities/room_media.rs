use crate::db::schema::room_media;
use crate::messages::error::ErrorResponse;
use crate::models::media::Media as MediaModel;
use serde::{Deserialize, Serialize};
use std::convert::TryFrom;
use uuid::Uuid;

#[derive(Clone, Debug, Deserialize, Identifiable, Insertable, Queryable, Serialize)]
#[table_name = "room_media"]
pub struct RoomMedia {
    pub id: String,
    pub room_id: String,
    pub source: String,
    pub name: String,
    pub room_media_index: i32,
    pub seconds: i32,
    pub url: String,
}

impl RoomMedia {
    pub fn new(model: MediaModel, room_id: &str) -> Result<Self, ErrorResponse> {
        let room_media_index = i32::try_from(model.index).map_err(|_| ErrorResponse::InternalServerError)?;
        let seconds =
            i32::try_from(model.seconds).map_err(|_| ErrorResponse::InternalServerError)?;

        Ok(Self {
            id: Uuid::new_v4().to_string(),
            room_id: room_id.to_string(),
            source: model.source.to_string(),
            name: model.name,
            room_media_index,
            seconds,
            url: model.url,
        })
    }
}
