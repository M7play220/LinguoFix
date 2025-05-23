// src/services/translate/mod.rs

use async_trait::async_trait;

use crate::services::deepl; // Імпортуємо модуль deepl

#[derive(Debug)]
pub enum TranslationError {
    ExternalApiError(String),
    UnsupportedLanguage(String),
    // Додайте інші типи помилок перекладу за потреби
}

impl std::fmt::Display for TranslationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TranslationError::ExternalApiError(msg) => write!(f, "External API Error: {}", msg),
            TranslationError::UnsupportedLanguage(lang) => {
                write!(f, "Unsupported Language: {}", lang)
            }
        }
    }
}

impl std::error::Error for TranslationError {}

#[async_trait]
pub trait TextTranslator: Send + Sync {
    async fn translate(
        &self,
        text: String,
        source_language: Option<String>,
        target_language: String,
        format: Option<String>,
    ) -> Result<String, TranslationError>;
}
