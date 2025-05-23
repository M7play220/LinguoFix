// src/services/correction.rs

use async_trait::async_trait;

#[derive(Debug)]
pub enum CorrectionError {
    ExternalApiError(String),
    // Додайте інші типи помилок корекції за потреби
}

impl std::fmt::Display for CorrectionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CorrectionError::ExternalApiError(msg) => write!(f, "External API Error: {}", msg),
        }
    }
}

impl std::error::Error for CorrectionError {}

#[async_trait]
pub trait TextCorrector: Send + Sync {
    async fn correct(
        &self,
        text: String,
        language: Option<String>,
    ) -> Result<String, CorrectionError>;
}
