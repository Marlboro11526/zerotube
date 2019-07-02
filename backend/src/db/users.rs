use crate::db::entities::{
    confirmation_email::ConfirmationEmail as DbConfirmationEmail, user::User as DbUser,
};
use crate::messages::error::ErrorResponse;
use crate::models::user::User;
use diesel::{prelude::*, r2d2::ConnectionManager};
use r2d2::PooledConnection;
use uuid::Uuid;

type Connection = PooledConnection<ConnectionManager<SqliteConnection>>;

pub fn get_user_with_email(
    email_input: &str,
    connection: &Connection,
) -> Result<Option<User>, ErrorResponse> {
    use crate::db::schema::users::dsl::*;

    users
        .filter(email.eq(email_input))
        .load::<DbUser>(connection)
        .map_err(|err| ErrorResponse::BadRequest("Invalid email".into()))
        .and_then(|mut result| {
            if let Some(user) = result.pop() {
                Ok(Some(user.into()))
            } else {
                Ok(None)
            }
        })
}

pub fn create_user_and_confirmation_email(
    user: User,
    connection: &Connection,
) -> Result<String, ErrorResponse> {
    use crate::db::schema::{
        confirmation_emails::{self, dsl::*},
        users::{self, dsl::*},
    };

    let user = DbUser::new(user);
    let confirmation_email = DbConfirmationEmail::new(&user);

    diesel::insert_into(users::table)
        .values(user.clone())
        .execute(connection)
        .map_err(|err| {
            log::error!(
                "Failed to insert user into DB.\nError: {}\nUser: {:?}",
                err,
                user
            );

            ErrorResponse::InternalServerError
        })?;

    diesel::insert_into(confirmation_emails::table)
        .values(confirmation_email.clone())
        .execute(connection)
        .map_err(|err| {
            log::error!(
                "Failed to insert email into DB.\nError: {}\nEmail: {:?}",
                err,
                confirmation_email
            );

            ErrorResponse::InternalServerError
        })?;

    Ok(user.email)
}
