use crate::messages::error::ErrorResponse;
use argon2rs::verifier::Encoded;
use rand::RngCore;

pub fn compare_password(raw_password: &str, hashed_password: &str) -> Result<bool, ErrorResponse> {
    let encoded = Encoded::from_u8(hashed_password.as_bytes()).map_err(|_| {
        log::error!("Unable to reconstruct hash session for {}", hashed_password);

        ErrorResponse::InternalServerError
    })?;

    Ok(encoded.verify(raw_password.as_bytes()))
}

pub fn hash_password(password: &str) -> Result<String, ErrorResponse> {
    let salt = format!("{:X}", rand::thread_rng().next_u64());
    let encoded = Encoded::default2i(password.as_bytes(), salt.as_bytes(), &[], &[]);

    String::from_utf8(encoded.to_u8()).map_err(|_| ErrorResponse::InternalServerError)
}
