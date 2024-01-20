use std::fmt::Formatter;
use actix_web::{error, HttpResponse};
use actix_web::http::header::ContentType;
use log::error;
use serde::{Serialize};
use sqlx::Error;

const INTERNAL_ERROR_CODE: u32 = 1;
const DATABASE_ERROR_CODE: u32 = 2;

#[derive(Debug, Clone, Copy, Serialize)]
#[serde(rename_all = "snake_case")]
pub(crate) struct ErrorResponse<'a> {
    pub code: u32,
    pub message: &'a str,
}

impl<'a> ErrorResponse<'a> {
    pub fn new(code: u32, message: &'a str) -> Self {
        ErrorResponse {
            code,
            message,
        }
    }
}


impl std::fmt::Display for ErrorResponse<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match serde_json::to_string_pretty(self) {
            Ok(pretty) => write!(f, "{:}", pretty),
            Err(e) => write!(f, "{:}", e),
        }
    }
}

impl error::ResponseError for ErrorResponse<'_> {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::json())
            .body(self.to_string())
    }
}

impl<'a> From<sqlx::Error> for ErrorResponse<'a> {
    fn from(value: Error) -> Self {
        error!("sqlx::Error<{}>", value.to_string());
        ErrorResponse::<'a>::new(DATABASE_ERROR_CODE, "database error")
    }
}


impl<'a> ErrorResponse<'a> {
    pub const INTERNAL_ERROR: ErrorResponse<'static> = ErrorResponse { code: INTERNAL_ERROR_CODE, message: "Internal Error" };

    pub const AUTHORIZATION_ERROR: ErrorResponse<'static> = ErrorResponse { code: 2, message: "Authorization Error" };


}