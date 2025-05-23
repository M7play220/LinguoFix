// src/services/translate/deepl.rs

use crate::services::translate::{TextTranslator, TranslationError};
use async_trait::async_trait;
use reqwest::Client;
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
struct DeepLRequest {
    text: Vec<String>,
    #[serde(rename = "source_lang", skip_serializing_if = "Option::is_none")]
    source_lang: Option<String>,
    #[serde(rename = "target_lang")]
    target_lang: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    format: Option<String>,
}

#[derive(Deserialize)]
struct DeepLResponse {
    translations: Vec<DeepLTranslation>,
}

#[derive(Deserialize)]
struct DeepLTranslation {
    text: String,
    detected_source_language: String,
}

pub struct DeepLTranslator {
    api_key: String,
    client: Client,
}

impl DeepLTranslator {
    pub fn new(api_key: String) -> Self {
        DeepLTranslator {
            api_key,
            client: Client::new(),
        }
    }
}

#[async_trait]
impl TextTranslator for DeepLTranslator {
    async fn translate(
        &self,
        text: String,
        source_language: Option<String>,
        target_language: String,
        format: Option<String>,
    ) -> Result<String, TranslationError> {
        let api_url = "https://api-free.deepl.com/v2/translate"; // Або "https://api.deepl.com/v2/translate" для платного

        let request_body = DeepLRequest {
            text: vec![text],
            source_lang: source_language,
            target_lang: target_language,
            format,
        };

        let response = self
            .client
            .post(api_url)
            .header("Authorization", format!("DeepL-Auth-Key {}", self.api_key))
            .json(&request_body)
            .send()
            .await
            .map_err(|e| TranslationError::ExternalApiError(format!("Network error: {}", e)))?;

        if !response.status().is_success() {
            let status = response.status();
            let error_text = response
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error from DeepL API".to_string());
            eprintln!(
                "DeepL API error response (Status: {}): {}",
                status, error_text
            );
            return Err(TranslationError::ExternalApiError(format!(
                "DeepL API returned error status {}: {}",
                status, error_text
            )));
        }

        let deepl_response: DeepLResponse = response.json().await.map_err(|e| {
            TranslationError::ExternalApiError(format!("Failed to parse DeepL response: {}", e))
        })?;

        deepl_response
            .translations
            .into_iter()
            .next()
            .map(|t| t.text)
            .ok_or_else(|| {
                TranslationError::ExternalApiError(
                    "DeepL API did not return a translation".to_string(),
                )
            })
    }
}
