use crate::db::entities::{
    confirmation_email::ConfirmationEmail as DbConfirmationEmail, user::User as DbUser,
};
use crate::messages::error::ErrorResponse;
use crate::models::user::User;
use diesel::{prelude::*, r2d2::ConnectionManager, Connection as DieselConnection};
use r2d2::PooledConnection;

type Connection = PooledConnection<ConnectionManager<SqliteConnection>>;

pub fn get_user_with_username(
    username_input: &str,
    connection: &Connection,
) -> Result<Option<User>, ErrorResponse> {
    use crate::db::schema::users::dsl::*;

    let mut result = users
        .filter(username.eq(username_input))
        .load::<DbUser>(connection)?;

    Ok(result.pop().map(|entity| User::from(entity)))
}

pub fn get_user_with_email(
    email_input: &str,
    connection: &Connection,
) -> Result<Option<User>, ErrorResponse> {
    use crate::db::schema::users::dsl::*;

    let mut result = users
        .filter(email.eq(email_input))
        .load::<DbUser>(connection)?;

    Ok(result.pop().map(|entity| User::from(entity)))
}

pub fn create_user_and_confirmation_email(
    user: User,
    connection: &Connection,
) -> Result<(), ErrorResponse> {
    use crate::db::schema::{confirmation_emails, users};

    let user = DbUser::new(user);
    let confirmation_email = DbConfirmationEmail::new(&user);

    connection.transaction(|| {
        diesel::insert_into(users::table)
            .values(user)
            .execute(connection)?;

        diesel::insert_into(confirmation_emails::table)
            .values(confirmation_email)
            .execute(connection)?;

        Ok(())
    })
}

pub fn confirm_registration(
    confirmation_id: String,
    connection: &Connection,
) -> Result<(), ErrorResponse> {
    let confirmation_email = get_confirmation_email_by_id(confirmation_id, connection)?
        .ok_or(ErrorResponse::NotFound)?;

    update_user_activity_by_id(confirmation_email.user_id, true, connection)?;

    Ok(())
}

fn get_confirmation_email_by_id(
    confirmation_id: String,
    connection: &Connection,
) -> Result<Option<DbConfirmationEmail>, ErrorResponse> {
    use crate::db::schema::confirmation_emails::dsl::*;

    let mut result = confirmation_emails
        .filter(id.eq(&confirmation_id))
        .load::<DbConfirmationEmail>(connection)?;

    Ok(result.pop())
}

fn update_user_activity_by_id(
    user_id: String,
    new_active: bool,
    connection: &Connection,
) -> Result<(), ErrorResponse> {
    use crate::db::schema::users::dsl::*;

    let user = users.filter(id.eq(&user_id));

    diesel::update(user)
        .set(active.eq(new_active))
        .execute(connection)
        .map(|_| Ok(()))?
}
