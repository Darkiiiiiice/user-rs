use std::fmt::Formatter;
use actix_web::{error, HttpResponse};
use actix_web::http::header::ContentType;
use serde::{Serialize};

#[derive(Debug, Clone, Copy, Serialize)]
#[serde(rename_all = "snake_case")]
pub(crate) struct ErrorResponse {
    pub code: u32,
    pub message: &'static str,
}

impl std::fmt::Display for ErrorResponse {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match serde_json::to_string_pretty(self) {
            Ok(pretty) => write!(f, "{:}", pretty),
            Err(e) => write!(f, "{:}", e),
        }
    }
}

impl error::ResponseError for ErrorResponse {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::json())
            .body(self.to_string())
    }
}


impl ErrorResponse {
    pub const INTERNAL_ERROR: ErrorResponse = ErrorResponse { code: 1, message: "Internal Error" };
}