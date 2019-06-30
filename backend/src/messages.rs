use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    pub error: String,
}

#[derive(Deserialize)]
pub struct LoginRequest {
    pub password: String,
    pub username: String,
}

#[derive(Deserialize)]
pub struct RegisterRequest {
    pub email: String,
    pub password: String,
    pub username: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct UserSession {
    pub token: Option<String>,
    pub username: Option<String>,
}
