use crate::models::media::Media;
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct GetAllMediaResponse {
    pub media: Vec<Media>,
}

#[derive(Deserialize)]
pub struct AddMediaRequest {
    pub url: String,
}

#[derive(Deserialize)]
pub struct RemoveMediaRequest {
    pub url: String,
}
