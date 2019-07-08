use crate::db::entities::user::User as DbUser;
use crate::messages::auth::RegisterRequest;

#[derive(Clone, Debug)]
pub struct User {
    pub email: String,
    pub password: String,
    pub username: String,
}

impl From<DbUser> for User {
    fn from(entity: DbUser) -> Self {
        Self {
            email: entity.email,
            password: entity.password,
            username: entity.username,
        }
    }
}

impl From<RegisterRequest> for User {
    fn from(request: RegisterRequest) -> Self {
        Self {
            email: request.email,
            password: request.password,
            username: request.username,
        }
    }
}
