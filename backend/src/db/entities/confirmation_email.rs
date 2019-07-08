use crate::db::{
    entities::user::User,
    schema::confirmation_emails,
};
use chrono::{Duration, Local, NaiveDateTime};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, Deserialize, Insertable, Queryable, Serialize)]
#[table_name = "confirmation_emails"]
pub struct ConfirmationEmail {
    pub id: String,
    pub expiry_date_time: NaiveDateTime,
    pub user_id: String,
}

impl ConfirmationEmail {
    pub fn new(user: &User) -> Self {
        ConfirmationEmail {
            id: Uuid::new_v4().to_string(),
            expiry_date_time: Local::now().naive_local() + Duration::hours(24),
            user_id: user.id.clone(),
        }
    }
}
