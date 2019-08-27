use crate::messages::error::ErrorResponse;
use crate::models::media::Media;
use awc::Client;
use futures::future::Future;
use std::env;

pub fn get_media_youtube(id: &str) -> Result<Media, ErrorResponse> {
    let url = format!(
        "https://www.googleapis.com/youtube/v3/videos?id={}&key={}&part=contentDetails,snippet",
        id,
        env::var("YT_API_KEY").expect("Missing YT_API_KEY in env")
    );

    log::info!("{}", url);

    Client::new()
        .get(url)
        .send()
        .map_err(|error| {
            println!("Error: {:?}", error);
        })
        .and_then(|response| {
            println!("Response: {:?}", response);
            Ok(())
        })
        .wait();

    log::info!("DONE");

    unimplemented!()
}
