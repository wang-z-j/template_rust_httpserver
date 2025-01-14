use actix_web::{HttpResponse, ResponseError};
use std::fmt;

#[derive(Debug)]
pub struct MyError {
    pub message: String,
}

impl MyError {
    pub fn new(message: &str) -> Self {
        MyError {
            message: message.to_string(),
        }
    }
}

impl fmt::Display for MyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "MyError: {}", self.message)
    }
}

impl ResponseError for MyError {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::InternalServerError().body(self.message.clone())
    }
}
