use crate::db::schema::users;
use crate::models::user::User as UserModel;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, Deserialize, Identifiable, Insertable, Queryable, Serialize)]
#[table_name = "users"]
pub struct User {
    pub id: String,
    pub active: bool,
    pub email: String,
    pub password: String,
    pub username: String,
}

impl User {
    pub fn new(model: UserModel) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            active: false,
            email: model.email,
            password: model.password,
            username: model.username,
        }
    }
}
