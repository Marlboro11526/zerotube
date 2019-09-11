use crate::models::media::Media;
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct GetAllMediaResponse {
    pub media: Vec<Media>,
}

#[derive(Deserialize)]
pub struct AddMediaRequest {
    pub current: u32,
    pub location: AddMediaLocation,
    pub url: String,
}

#[derive(Deserialize)]
pub enum AddMediaLocation {
    Next,
    Last,
}

#[derive(Deserialize)]
pub struct RemoveMediaRequest {
    pub url: String,
}
