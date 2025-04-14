use crate::models::common::*;
use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

// --- Ответы ---

/// Продажа партнера в разрезе валюты.
#[derive(Deserialize, Debug, Clone)]
pub struct PartnerSaleDto {
    pub currency: CurrencyDto,
    /// Количество проданных экземпляров.
    pub count: i64,
    /// Общая сумма продаж в данной валюте.
    #[serde(rename = "amountTotal")]
    pub amount_total: f64, // В OpenAPI `integer`, но вероятнее `number`/`f64`
}

/// Продажи партнера сгруппированные по продукту (ответ GET /api/v1/sales/).
#[derive(Deserialize, Debug, Clone)]
pub struct PartnerProductDto {
    #[serde(rename = "productId")]
    pub product_id: Uuid,
    pub title: Option<String>, // В OpenAPI обязателен
    /// Статус продукта (тип не специфицирован, используем String).
    pub status: Option<String>,
    /// Список продаж по валютам.
    #[serde(default)]
    pub sales: Vec<PartnerSaleDto>,
}

/// Детализация продажи партнера по конкретному продукту (элемент ответа GET /api/v1/sales/{productId}).
#[derive(Deserialize, Debug, Clone)]
pub struct PartnerSaleDetailsDto {
    /// Идентификатор контракта.
    pub id: Uuid,
    /// Дата создания контракта.
    #[serde(rename = "createdAt")]
    pub created_at: Option<DateTime<Utc>>, // В OpenAPI обязателен
    /// Статус контракта.
    pub status: Option<ContractStatusDto>, // В OpenAPI обязателен
    /// Сумма контракта.
    #[serde(rename = "amountTotal")]
    pub amount_total: Option<AmountTotalDto>, // В OpenAPI обязателен
    /// Информация о покупателе.
    pub buyer: Option<BuyerDto>, // В OpenAPI обязателен
}

// --- Пагинированные ответы ---

/// Пагинированный ответ для GET /api/v1/sales/.
#[derive(Deserialize, Debug, Clone)]
pub struct PartnerSalesPageResponse {
    pub items: Vec<PartnerProductDto>,
    pub total: i64,
    pub page: i64,
    pub size: i64,
    #[serde(rename = "totalPages")]
    pub total_pages: i64,
}

/// Пагинированный ответ для GET /api/v1/sales/{productId}.
#[derive(Deserialize, Debug, Clone)]
pub struct PartnerProductSalesPageResponse {
    pub items: Vec<PartnerSaleDetailsDto>,
    pub total: i64,
    pub page: i64,
    pub size: i64,
    #[serde(rename = "totalPages")]
    pub total_pages: i64,
}

// --- Параметры запросов (Query) ---

/// Параметры для запроса GET /api/v1/sales/.
#[derive(Serialize, Debug, Default, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ListPartnerSalesParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<i64>,
}

/// Параметры для запроса GET /api/v1/sales/{productId}.
#[derive(Serialize, Debug, Default, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ListPartnerProductSalesParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<i64>,
    /// Начало периода продаж (формат "YYYY-MM-DD").
    #[serde(
        skip_serializing_if = "Option::is_none",
        with = "opt_chrono_naive_date_as_str",
        default
    )]
    pub from_date: Option<NaiveDate>,
    /// Конец периода продаж (формат "YYYY-MM-DD").
    #[serde(
        skip_serializing_if = "Option::is_none",
        with = "opt_chrono_naive_date_as_str",
        default
    )]
    pub to_date: Option<NaiveDate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<CurrencyDto>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<ContractStatusDto>,
    /// Строка для поиска.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search: Option<String>,
}
