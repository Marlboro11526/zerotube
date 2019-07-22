use crate::models::room::Room;
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct RoomCreateRequest {
    pub description: String,
    pub name: String,
    pub public: bool,
    pub url: String,
}

#[derive(Serialize)]
pub struct RoomGetAllResponse {
    pub rooms: Vec<Room>,
}
