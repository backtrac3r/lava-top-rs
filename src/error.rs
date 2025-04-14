use chrono::{DateTime, Utc};
use reqwest::StatusCode;
use serde::Deserialize;
use thiserror::Error;

/// Структура для деталей ошибки из API Lava Top (соответствует ErrorResponse).
#[derive(Deserialize, Debug, Clone)]
pub struct ApiErrorDetails {
    pub error: Option<String>,
    /// Детальное описание ошибки, часто объект с полями.
    pub details: Option<serde_json::Value>,
    pub timestamp: Option<DateTime<Utc>>,
}

/// Перечисление ошибок, которые могут возникнуть при работе с Lava Top API.
#[derive(Error, Debug)]
pub enum LavaTopError {
    /// Ошибка сети или ошибка во время выполнения HTTP запроса.
    #[error("Сетевая ошибка или ошибка HTTP запроса: {0}")]
    Reqwest(#[from] reqwest::Error),

    /// Ошибка сериализации данных в JSON или десериализации из JSON.
    #[error("Ошибка сериализации/десериализации JSON: {0}")]
    Serde(#[from] serde_json::Error),

    /// Ошибка парсинга URL.
    #[error("Ошибка парсинга URL: {0}")]
    UrlParse(#[from] url::ParseError),

    /// Ошибка создания значения HTTP заголовка.
    #[error("Неверное значение заголовка: {0}")]
    InvalidHeaderValue(#[from] reqwest::header::InvalidHeaderValue),

    /// Ошибка, возвращенная API Lava Top (неуспешный HTTP статус).
    #[error("Ошибка API Lava Top (Статус: {status}): {details:?}{}", raw_body.as_deref().map(|b| format!("\nТело ответа: {}", b)).unwrap_or_default())]
    ApiError {
        status: StatusCode,
        details: Option<ApiErrorDetails>,
        raw_body: Option<String>, // Сохраняем сырое тело на случай ошибки парсинга деталей
    },

    /// Отсутствует необходимое поле в ответе API.
    #[error("Отсутствует обязательное поле в ответе: {0}")]
    MissingField(String),

    /// Неверный формат или значение параметра в запросе к API.
    #[error("Неверный формат параметра запроса: {0}")]
    InvalidQueryParam(String),

    /// Не был предоставлен обязательный параметр для вызова метода API.
    #[error("Отсутствует обязательный параметр в запросе: {0}")]
    MissingParameter(String),
}
