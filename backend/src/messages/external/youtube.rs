use serde::Deserialize;

#[derive(Deserialize)]
pub struct YoutubeVideoListResponse {
    pub items: Vec<YoutubeVideoItem>,
}

#[derive(Deserialize)]
pub struct YoutubeVideoItem {
    pub id: String,
    pub snippet: YoutubeVideoItemSnippet,
    pub content_details: YoutubeVideoItemContentDetails,
}

#[derive(Deserialize)]
pub struct YoutubeVideoItemSnippet {
    pub title: String,
}

#[derive(Deserialize)]
pub struct YoutubeVideoItemContentDetails {
    pub duration: String,
}
