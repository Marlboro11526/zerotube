use crate::models::room::Room;
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct RoomCreateRequest {
    pub name: String,
    pub public: bool,
}

#[derive(Serialize)]
pub struct RoomGetAllResponse {
    pub rooms: Vec<Room>,
}
