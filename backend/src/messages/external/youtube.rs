use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct YoutubeVideoListResponse {
    pub items: Vec<YoutubeVideoItem>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct YoutubeVideoItem {
    pub id: String,
    pub snippet: YoutubeVideoItemSnippet,
    pub content_details: YoutubeVideoItemContentDetails,
}

#[derive(Debug, Deserialize)]
pub struct YoutubeVideoItemSnippet {
    pub title: String,
}

#[derive(Debug, Deserialize)]
pub struct YoutubeVideoItemContentDetails {
    pub duration: String,
}
