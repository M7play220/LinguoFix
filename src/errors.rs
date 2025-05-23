// src/errors.rs

use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;

#[derive(Debug)]
pub enum CustomError {
    TextGearsError(String),
    DeepLError(String),
    TranslationError(String),
    UnsupportedLanguage(String),
    InternalServerError(String),
    // Додайте інші типи помилок за потреби
}

impl IntoResponse for CustomError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            CustomError::TextGearsError(msg) => (
                StatusCode::BAD_GATEWAY,
                format!("TextGears API Error: {}", msg),
            ),
            CustomError::DeepLError(msg) => (
                StatusCode::BAD_GATEWAY,
                format!("DeepL API Error: {}", msg),
            ),
            CustomError::TranslationError(msg) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Translation Error: {}", msg),
            ),
            CustomError::UnsupportedLanguage(lang) => (
                StatusCode::BAD_REQUEST,
                format!("Unsupported Language: {}", lang),
            ),
            CustomError::InternalServerError(msg) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Internal Server Error: {}", msg),
            ),
        };

        let body = Json(json!({
            "error": error_message,
        }));

        (status, body).into_response()
    }
}