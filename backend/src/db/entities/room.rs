use crate::db::schema::rooms;
use crate::models::room::Room as RoomModel;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, Deserialize, Identifiable, Insertable, Queryable, Serialize)]
#[table_name = "rooms"]
pub struct Room {
    pub id: String,
    pub name: String,
    pub public: bool,
}

impl Room {
    pub fn new(model: RoomModel) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            name: model.name,
            public: model.public,
        }
    }
}
