use crate::messages::{error::ErrorResponse, external::youtube::YoutubeVideoListResponse};
use crate::models::media::{Media, MediaSource};
use crate::services::duration;
use reqwest::blocking as client;
use std::env;

pub fn get_media_youtube(id: &str, index: u32) -> Result<Media, ErrorResponse> {
    let url = format!(
        "https://www.googleapis.com/youtube/v3/videos?id={}&key={}&part=contentDetails,snippet",
        id,
        env::var("YT_API_KEY").expect("Missing YT_API_KEY in env")
    );

    let response = client::get(&url).map_err(|_| ErrorResponse::ServiceUnavailable)?;

    let response = response
        .json::<YoutubeVideoListResponse>()
        .map_err(|_| ErrorResponse::InternalServerError)?;

    let video = response
        .items
        .get(0)
        .ok_or(ErrorResponse::InternalServerError)?;

    Ok(Media {
        index,
        name: video.snippet.title.clone(),
        seconds: duration::duration_to_seconds(&video.content_details.duration)?,
        source: MediaSource::YouTube,
        url: id.to_string(),
    })
}
