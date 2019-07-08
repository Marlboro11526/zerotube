use crate::db::entities::room::Room as DbRoom;
use crate::messages::room::RoomCreateRequest;
use serde::Serialize;

#[derive(Clone, Debug, Serialize)]
pub struct Room {
    pub name: String,
    pub public: bool,
}

impl From<DbRoom> for Room {
    fn from(entity: DbRoom) -> Self {
        Self {
            name: entity.name,
            public: entity.public,
        }
    }
}

impl From<RoomCreateRequest> for Room {
    fn from(request: RoomCreateRequest) -> Self {
        Self {
            name: request.name,
            public: request.public,
        }
    }
}
