// src/services/textgears_corrector.rs

use crate::services::correction::CorrectionError;
use crate::services::correction::TextCorrector;
use async_trait::async_trait;
use reqwest::Client;
use serde::Deserialize;
use urlencoding;

// Допоміжні структури для десериалізації відповіді TextGears API
#[derive(Debug, Deserialize)]
struct Description {
    en: String, // Або HashMap<String, String> якщо мова може змінюватися
}

#[derive(Debug, Deserialize)]
struct ErrorDetail {
    bad: String,
    better: Vec<String>,
    offset: usize,
    length: usize,
    description: Option<Description>, // Змінено на Option<Description> на випадок, якщо description відсутній
    #[serde(default)] // Додано для ігнорування невідомих полів
    r#type: String, // Використовуємо r#type, бо type - зарезервоване слово
}

#[derive(Debug, Deserialize)]
struct ApiResponse {
    result: bool,
    errors: Vec<ErrorDetail>,
    #[serde(flatten)] // Для обробки інших полів, які ми не визначаємо явно
    _extra: std::collections::HashMap<String, serde_json::Value>,
}

pub struct TextGearsCorrector {
    api_key: String,
}

impl TextGearsCorrector {
    pub fn new(api_key: String) -> Self {
        TextGearsCorrector { api_key }
    }
}

async fn fetch_corrections_from_textgears(
    text: &str,
    api_key: &str,
) -> Result<Vec<(String, String, usize, usize)>, CorrectionError> {
    let url = format!(
        "https://api.textgears.com/check.php?text={}&key={}",
        urlencoding::encode(text),
        api_key
    );

    let client = Client::new();
    let response = client
        .get(&url)
        .send()
        .await
        .map_err(|e| CorrectionError::ExternalApiError(format!("Network error: {}", e)))?;

    let response_text = response.text().await.map_err(|e| {
        CorrectionError::ExternalApiError(format!("Failed to read response text: {}", e))
    })?;

    println!("Необроблена відповідь API TextGears: {}", response_text);

    let api_response: ApiResponse = serde_json::from_str(&response_text).map_err(|e| {
        CorrectionError::ExternalApiError(format!("Failed to parse TextGears response: {}", e))
    })?;

    if !api_response.result {
        // Якщо API повертає result: false, це може бути помилка на стороні TextGears
        return Err(CorrectionError::ExternalApiError(
            "TextGears API returned result: false".to_string(),
        ));
    }

    let corrections: Vec<(String, String, usize, usize)> = api_response
        .errors
        .into_iter()
        .filter_map(|err| {
            // Перевіряємо, чи є хоча б одне виправлення
            err.better
                .first()
                .map(|suggestion| (err.bad, suggestion.clone(), err.offset, err.length))
        })
        .collect();

    Ok(corrections)
}

#[async_trait]
impl TextCorrector for TextGearsCorrector {
    async fn correct(
        &self,
        text: String,
        _language: Option<String>,
    ) -> Result<String, CorrectionError> {
        // Отримуємо виправлення від TextGears
        let corrections = fetch_corrections_from_textgears(&text, &self.api_key).await?;

        let mut corrected_text = text.to_string();

        // Застосовуємо виправлення в зворотному порядку, щоб уникнути проблем зі зсувом індексів
        // У Rust String::replace_range працює з байтовими індексами, тому для UTF-8 тексту
        // необхідно бути обережним. Для простих сценаріїв з латиницею це працює.
        // Для більш надійного рішення для UTF-8 тексту, варто використовувати обхід по символах або grapheme clusters.
        // Але для більшості випадків, коли довжина заміни не сильно відрізняється і мова латиниця, це може спрацювати.

        // Ми використовуємо `corrected_text.get(start..end).map_or("", |s| s) == bad`
        // для перевірки, чи оригінальний сегмент тексту досі відповідає "bad"
        // (що може бути порушено попередніми змінами), але це необхідно для безпеки.
        // Якщо `offset` та `length` від TextGears завжди відносяться до оригінального тексту,
        // а не до поточного стану тексту, то зворотний порядок є ключовим.

        for (bad, better, offset, length) in corrections.into_iter().rev() {
            let start = offset;
            let end = start + length;

            // Перевіряємо, чи діапазон дійсний і чи текст у ньому збігається з "bad"
            // Це важливо, оскільки попередні заміни могли змінити текст або довжину.
            if let Some(current_segment) = corrected_text.get(start..end) {
                if current_segment == bad {
                    // Виконуємо заміну
                    corrected_text.replace_range(start..end, &better);
                } else {
                    // Якщо сегмент не збігається, це може бути через попередні зміни.
                    // Для складнішої логіки можна спробувати знайти "bad" слово знову
                    // або пропустити це виправлення.
                    // println!("Попередження: Текст '{}' не збігається з очікуваним '{}' за офсетом {}. Пропускаємо виправлення.", current_segment, bad, start);
                }
            } else {
                // println!("Попередження: Недійсний діапазон для заміни [{}..{}] у тексті довжиною {}. Пропускаємо виправлення.", start, end, corrected_text.len());
            }
        }

        Ok(corrected_text)
    }
}
