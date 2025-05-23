// src/models.rs

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct CorrectTextRequest {
    pub text: String,
    pub language: Option<Language>,
}

#[derive(Debug, Serialize)]
pub struct CorrectTextResponse {
    pub corrected_text: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "lowercase")] // Це дозволить десеріалізувати "uk", "en", "de" тощо.
pub enum Language {
    Uk, // Ukrainian
    En, // English (default for auto-detection if not specified by TextGear)
    De, // German
    Fr, // French
    Es, // Spanish
    It, // Italian
    Pl, // Polish
    Pt, // Portuguese (Portugal and Brazil)
        // Додайте інші мови, які підтримує TextGear API
}

impl ToString for Language {
    fn to_string(&self) -> String {
        match self {
            Language::Uk => "uk".to_string(),
            Language::En => "en".to_string(),
            Language::De => "de".to_string(),
            Language::Fr => "fr".to_string(),
            Language::Es => "es".to_string(),
            Language::It => "it".to_string(),
            Language::Pl => "pl".to_string(),
            Language::Pt => "pt".to_string(),
        }
    }
}
