use crate::messages::error::ErrorResponse;
use crate::models::media::Media;
use std::env;

pub fn get_media_youtube(id: &str) -> Result<Media, ErrorResponse> {
    let url = format!(
        "https://www.googleapis.com/youtube/v3/videos?id={}&key={}&part=contentDetails,snippet",
        id,
        env::var("YT_API_KEY").expect("Missing YT_API_KEY in env")
    );

    log::info!("{}", url);

    unimplemented!()
}
