use crate::db::entities::{
    confirmation_email::ConfirmationEmail as DbConfirmationEmail, user::User as DbUser,
};
use crate::messages::error::ErrorResponse;
use crate::models::user::User;
use diesel::{prelude::*, r2d2::ConnectionManager};
use r2d2::PooledConnection;

type Connection = PooledConnection<ConnectionManager<SqliteConnection>>;

pub fn get_user_with_username(
    username_input: &str,
    connection: &Connection,
) -> Result<User, ErrorResponse> {
    use crate::db::schema::users::dsl::*;

    users
        .filter(username.eq(username_input))
        .load::<DbUser>(connection)
        .map_err(|err| {
            log::error!(
                "Failed to load user from DB.\nError: {}\nUsername: {}",
                err,
                username_input,
            );

            ErrorResponse::BadRequest("Invalid username".into())
        })
        .and_then(|mut result| {
            if let Some(user) = result.pop() {
                Ok(user.into())
            } else {
                Err(ErrorResponse::BadRequest("Username not found".into()))
            }
        })
}

pub fn get_user_with_email(
    email_input: &str,
    connection: &Connection,
) -> Result<Option<User>, ErrorResponse> {
    use crate::db::schema::users::dsl::*;

    users
        .filter(email.eq(email_input))
        .load::<DbUser>(connection)
        .map_err(|err| {
            log::error!(
                "Failed to load user from DB.\nError: {}\nEmail: {}",
                err,
                email_input
            );

            ErrorResponse::BadRequest("Invalid email".into())
        })
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
) -> Result<(), ErrorResponse> {
    use crate::db::schema::{confirmation_emails, users};

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

    Ok(())
}

pub fn confirm_registration(
    confirmation_id: String,
    connection: &Connection,
) -> Result<(), ErrorResponse> {
    let confirmation_email = get_confirmation_email_by_id(confirmation_id, connection)?;
    let updated_rows = update_user_activity_by_id(confirmation_email.user_id, true, connection)?;

    if updated_rows == 1 {
        Ok(())
    } else {
        Err(ErrorResponse::InternalServerError)
    }
}

fn get_confirmation_email_by_id(
    confirmation_id: String,
    connection: &Connection,
) -> Result<DbConfirmationEmail, ErrorResponse> {
    use crate::db::schema::confirmation_emails::dsl::*;

    confirmation_emails
        .filter(id.eq(&confirmation_id))
        .load::<DbConfirmationEmail>(connection)
        .map_err(|err| {
            log::error!(
                "Failed to load confirmation email from DB.\nError: {}\nID: {}",
                err,
                confirmation_id
            );

            ErrorResponse::BadRequest("Invalid ID".into())
        })
        .and_then(|mut result| {
            if let Some(email) = result.pop() {
                Ok(email)
            } else {
                Err(ErrorResponse::BadRequest("Invalid ID".into()))
            }
        })
}

fn update_user_activity_by_id(
    user_id: String,
    new_active: bool,
    connection: &Connection,
) -> Result<usize, ErrorResponse> {
    use crate::db::schema::users::dsl::*;

    let user = users.filter(id.eq(&user_id));

    diesel::update(user)
        .set(active.eq(new_active))
        .execute(connection)
        .map(Ok)
        .map_err(|err| {
            log::error!("Failed to update user.\nError: {}\nID: {}", err, user_id);

            ErrorResponse::BadRequest("Invalid ID".into())
        })?
}
