use crate::db::entities::room::Room as DbRoom;
use crate::messages::room::RoomCreateRequest;
use serde::Serialize;

#[derive(Clone, Debug, Serialize)]
pub struct Room {
    pub description: String,
    pub name: String,
    pub public: bool,
    pub url: String,
}

impl From<DbRoom> for Room {
    fn from(entity: DbRoom) -> Self {
        Self {
            description: entity.description,
            name: entity.name,
            public: entity.public,
            url: entity.url,
        }
    }
}

impl From<RoomCreateRequest> for Room {
    fn from(request: RoomCreateRequest) -> Self {
        Self {
            description: request.description,
            name: request.name,
            public: request.public,
            url: request.url,
        }
    }
}
