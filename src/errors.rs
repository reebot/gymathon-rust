use actix_web::{error::ResponseError, http::StatusCode};
use std::fmt::{Display, Formatter, Result as FmtResult};

#[derive(Debug)]
pub struct MyError {
    pub message: String,
}

impl Display for MyError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}", self.message)
    }
}

impl ResponseError for MyError {
    fn status_code(&self) -> StatusCode {
        // You might choose different status codes based on the error message
        StatusCode::INTERNAL_SERVER_ERROR
    }
}
