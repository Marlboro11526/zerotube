use crate::db::entities::room_media::RoomMedia as DbMedia;
use crate::messages::error::ErrorResponse;
use crate::services::media_apis;
use derive_more::Display;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::convert::TryFrom;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Media {
    pub index: u32,
    pub name: String,
    pub seconds: u32,
    pub source: MediaSource,
    pub url: String,
}

#[derive(Clone, Debug, Display, Deserialize, Serialize)]
pub enum MediaSource {
    #[display(fmt = "youtube")]
    YouTube,
}

impl Media {
    pub fn new(url: &str, index: u32) -> Result<Self, ErrorResponse> {
        let youtube =
            Regex::new(r#"(youtube\.com/watch\?v=|youtu\.be/)(?P<id>[A-Za-z0-9_-]{11})"#)?;

        if youtube.is_match(url) {
            media_apis::get_media_youtube(&youtube.captures(url).unwrap()["id"], index)
        } else {
            Err(ErrorResponse::BadRequest("Unknown media URL".into()))
        }
    }
}

impl From<DbMedia> for Media {
    fn from(entity: DbMedia) -> Self {
        let index = u32::try_from(entity.room_media_index).unwrap_or(0);
        let seconds = u32::try_from(entity.seconds).unwrap_or(0);

        Self {
            index,
            name: entity.name,
            seconds,
            source: MediaSource::from(entity.source),
            url: entity.url,
        }
    }
}

impl From<String> for MediaSource {
    fn from(s: String) -> Self {
        match s.as_str() {
            "youtube" => MediaSource::YouTube,
            _ => panic!("Unable to match '{}' to MediaSource", s),
        }
    }
}
