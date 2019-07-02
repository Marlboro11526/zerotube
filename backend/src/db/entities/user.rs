use crate::db::schema::{confirmation_emails, users};
use crate::models::user::User as UserModel;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, Deserialize, Insertable, Queryable, Serialize)]
#[table_name = "users"]
pub struct User {
    pub id: String,
    pub email: String,
    pub username: String,
    pub password: String,
    pub active: bool,
}

impl User {
    pub fn new(model: UserModel) -> Self {
        User {
            id: Uuid::new_v4().to_string(),
            email: model.email,
            username: model.username,
            password: model.password,
            active: false,
        }
    }
}
