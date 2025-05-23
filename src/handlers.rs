// src/handlers.rs

use crate::errors::CustomError;
use crate::services::correction::TextCorrector;
use crate::services::translate::TextTranslator;
use crate::services::translate::TranslationError;
use axum::Extension;
use axum::Json;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

// Тип для зберігання екземпляра TextCorrector в Extension
type Corrector = Arc<dyn TextCorrector + Send + Sync + 'static>;
// Тип для зберігання екземпляра TextTranslator в Extension
type Translator = Arc<dyn TextTranslator + Send + Sync + 'static>;

#[derive(Deserialize, Debug)]
pub struct CorrectTextRequest {
    pub text: String,
    pub language: Option<String>,
}

#[derive(Serialize)]
pub struct CorrectTextResponse {
    pub corrected_text: String,
}

pub async fn correct_text(
    Extension(corrector): Extension<Corrector>,
    Json(payload): Json<CorrectTextRequest>,
) -> Result<Json<CorrectTextResponse>, (axum::http::StatusCode, String)> {
    println!("Отримано запит на корекцію: {:?}", payload);
    match corrector
        .correct(payload.text.clone(), payload.language)
        .await
    {
        Ok(corrected) => {
            println!("Результат корекції: {}", corrected);
            Ok(Json(CorrectTextResponse {
                corrected_text: corrected,
            }))
        }
        Err(e) => {
            eprintln!("Помилка корекції: {}", e);
            Err((axum::http::StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))
        }
    }
}

#[derive(Deserialize, Debug)]
pub struct TranslateTextRequest {
    pub text: String,
    pub source_language: String, // Це String, як у вашій структурі
    pub target_language: String,
}

#[derive(Serialize)]
pub struct TranslateTextResponse {
    pub translated_text: String,
}

pub async fn translate_text(
    Extension(translator): Extension<Translator>,
    Json(payload): Json<TranslateTextRequest>,
) -> Result<Json<TranslateTextResponse>, (axum::http::StatusCode, String)> {
    println!("Отримано запит на переклад: {:?}", payload);

    match translator
        .translate(
            payload.text,
            Some(payload.source_language), // <--- ЗАГОРНУТО В Some()
            payload.target_language.clone(),
            None,
        )
        .await
    {
        Ok(translated_text) => Ok(Json(TranslateTextResponse { translated_text })),
        Err(err) => match err {
            TranslationError::UnsupportedLanguage(lang) => Err((
                axum::http::StatusCode::BAD_REQUEST,
                format!("Непідтримувана мова: {}", lang),
            )),
            other => Err((
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                format!("Помилка перекладу: {}", other),
            )),
        },
    }
}
