use crate::db::entities::room_media::RoomMedia as DbMedia;
use crate::messages::{error::ErrorResponse, external::youtube::YoutubeVideoItem};
use crate::services::{duration, media_apis};
use derive_more::Display;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::convert::TryFrom;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Media {
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
    pub fn new(url: &str) -> Result<Self, ErrorResponse> {
        let youtube = Regex::new(r#"(youtube\.com/watch\?v=|youtu\.be/)(?P<id>[A-Za-z_-]{11})"#)?;

        if youtube.is_match(url) {
            media_apis::get_media_youtube(&youtube.captures(url).unwrap()["id"])
        } else {
            Err(ErrorResponse::BadRequest("Unknown media URL".into()))
        }
    }
}

impl From<DbMedia> for Media {
    fn from(entity: DbMedia) -> Self {
        let seconds = u32::try_from(entity.seconds).unwrap_or(0);

        Self {
            name: entity.name,
            seconds,
            source: MediaSource::from(entity.source),
            url: entity.url,
        }
    }
}

impl TryFrom<YoutubeVideoItem> for Media {
    type Error = ErrorResponse;

    fn try_from(message: YoutubeVideoItem) -> Result<Self, Self::Error> {
        Ok(Self {
            name: message.snippet.title,
            seconds: duration::duration_to_seconds(&message.content_details.duration)?,
            source: MediaSource::YouTube,
            url: message.id,
        })
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
