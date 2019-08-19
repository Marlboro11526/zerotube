use crate::messages::error::ErrorResponse;
use iso8601::{self, Duration};

pub fn duration_to_seconds(s: &str) -> Result<u32, ErrorResponse> {
    let duration = iso8601::duration(s).map_err(|_| ErrorResponse::InternalServerError)?;

    if let Duration::YMDHMS {
        year,
        month,
        day,
        hour,
        minute,
        second,
        millisecond,
    } = duration
    {
        if day > 0 || month > 0 || year > 0 {
            return Err(ErrorResponse::BadRequest("Duration too long".into()));
        }

        Ok(second + (minute * 60) + (hour * 60 * 60))
    } else {
        Err(ErrorResponse::InternalServerError)
    }
}
