// src/main.rs

mod errors;
mod handlers;
mod models; // Може бути порожнім або містити спільні структури
mod services;

use crate::services::correction::TextCorrector;
use crate::services::deepl::DeepLTranslator;
use crate::services::textgears_corrector::TextGearsCorrector;
use crate::services::translate::TextTranslator;

use axum::{
    Extension, Router,
    routing::{get, post},
};
use dotenv::dotenv;
use http::header;
use std::{net::SocketAddr, sync::Arc};
use tokio::net::TcpListener;
use tower_http::cors::{AllowOrigin, CorsLayer};
use tower_http::services::ServeDir;

#[tokio::main]
async fn main() {
    dotenv().ok(); // Завантажуємо змінні середовища з .env

    let textgears_api_key = std::env::var("TEXTGEARS_API_KEY")
        .expect("TEXTGEARS_API_KEY має бути встановлено у змінних середовища");

    let deepl_api_key = std::env::var("DEEPL_API_KEY")
        .expect("DEEPL_API_KEY має бути встановлено у змінних середовища");

    // Ініціалізація TextCorrector
    let corrector: Arc<dyn TextCorrector + Send + Sync + 'static> =
        Arc::new(TextGearsCorrector::new(textgears_api_key));

    // Ініціалізація TextTranslator
    let translator: Arc<dyn TextTranslator + Send + Sync + 'static> =
        Arc::new(DeepLTranslator::new(deepl_api_key));

    // Налаштування CORS
    let cors = CorsLayer::new()
        .allow_origin(AllowOrigin::any()) // Для розробки. У продакшні вкажіть конкретні домени: .allow_origin(["http://localhost:3000".parse().unwrap()])
        .allow_methods(vec![http::Method::POST, http::Method::OPTIONS])
        .allow_headers(vec![header::CONTENT_TYPE, header::AUTHORIZATION]);

    // Роутер для API-ендпоінтів
    let api_router = Router::new()
        .route("/correct", post(handlers::correct_text)) // POST /api/correct
        .route("/translate", post(handlers::translate_text)) // POST /api/translate
        .route("/hello", get(|| async { "Привіт!" })) // GET /api/hello (приклад)
        .layer(Extension(corrector.clone())) // Додаємо коректор як розширення
        .layer(Extension(translator.clone())) // Додаємо перекладач як розширення
        .layer(cors); // Застосовуємо CORS до API-маршрутів

    // Роутер для статичних файлів (HTML, CSS, JS)
    let static_router = Router::new().nest_service("/", ServeDir::new("static"));

    // Головний роутер: об'єднуємо API та статичні файли
    let app = Router::new()
        .nest("/api", api_router) // Усі API-маршрути будуть починатися з /api
        .merge(static_router); // Статичні файли доступні з кореня

    // Запуск сервера
    let addr = SocketAddr::from(([127, 0, 0, 1], 8000));
    let listener = TcpListener::bind(addr).await.unwrap();
    println!("Сервер запущено на http://{}", addr);

    axum::serve(listener, app).await.unwrap();
}
