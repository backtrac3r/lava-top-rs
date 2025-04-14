use crate::models::common::*;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

// --- Запросы ---

/// Данные для обновления цены оффера.
#[derive(Serialize, Debug, Clone)]
pub struct UpdatePriceRequest {
    /// Новая цена.
    pub amount: f64,
    /// Валюта цены.
    pub currency: CurrencyDto,
}

/// Данные для обновления оффера.
#[derive(Serialize, Debug, Clone)]
pub struct UpdateOfferRequest {
    /// ID обновляемого оффера.
    pub id: Uuid,
    /// Новый список цен (если обновляются цены).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prices: Option<Vec<UpdatePriceRequest>>,
    /// Новое название оффера.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Новое описание оффера.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

/// Данные для обновления продукта (PATCH /api/v2/products/{productId}).
#[derive(Serialize, Debug, Clone)]
pub struct ProductUpdateRequest {
    /// Список обновляемых офферов.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offers: Option<Vec<UpdateOfferRequest>>,
}

// --- Ответы ---

/// Устаревшее поле, описывающее повторяемость платежа.
#[derive(Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
#[serde(rename_all = "lowercase")]
pub enum RecurrentDto {
    Monthly,
}

/// Предложение (оффер) для покупки продукта.
#[derive(Deserialize, Debug, Clone)]
pub struct OfferResponse {
    pub id: Uuid,
    pub name: Option<String>,
    pub description: Option<String>,
    /// Список цен в разных валютах.
    pub prices: Vec<PriceDto>,
    #[deprecated(note = "Используйте поле periodicity в PriceDto")]
    pub recurrent: Option<RecurrentDto>,
}

/// Описание продукта в ответе API.
#[derive(Deserialize, Debug, Clone)]
pub struct ProductItemResponse {
    pub id: Uuid,
    pub title: Option<String>,
    pub description: Option<String>,
    #[serde(rename = "type")]
    pub product_type: ProductType,
    #[serde(default)] // Поле offers может отсутствовать
    pub offers: Vec<OfferResponse>,
}

/// Описание поста в ответе API.
#[derive(Deserialize, Debug, Clone)]
pub struct PostItemResponse {
    pub id: Uuid,
    pub title: Option<String>, // Сделаем Option для надежности
    pub description: Option<String>,
    pub body: Option<String>,
    #[serde(rename = "type")]
    pub post_type: PostType,
    #[serde(rename = "createdAt")]
    pub created_at: Option<DateTime<Utc>>,
    #[serde(rename = "updatedAt")]
    pub updated_at: Option<DateTime<Utc>>,
    #[serde(rename = "publishedAt")]
    pub published_at: Option<DateTime<Utc>>,
}

/// Перечисление для представления либо продукта, либо поста в ответе /api/v2/products.
#[derive(Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum FeedData {
    Product(ProductItemResponse),
    Post(PostItemResponse),
}

/// Элемент ленты продуктов/постов (ответ /api/v2/products).
#[derive(Deserialize, Debug, Clone)]
pub struct FeedItemCombined {
    #[serde(rename = "type")]
    pub item_type: FeedItemType,
    pub data: FeedData,
}

// --- Параметры запросов (Query) ---

/// Параметры для запроса списка продуктов (GET /api/v2/products).
#[derive(Serialize, Debug, Default, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ListProductsParams {
    /// Искать элементы, созданные до этой даты.
    #[serde(
        skip_serializing_if = "Option::is_none",
        with = "opt_chrono_datetime_as_iso8601",
        default
    )]
    pub before_created_at: Option<DateTime<Utc>>,
    /// Фильтр по типу контента (POST или PRODUCT).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_categories: Option<FeedItemType>, // OpenAPI говорит schema: $ref, но примеры не массивы? Оставляем один
    /// Фильтр по типу продукта.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_types: Option<ProductType>, // OpenAPI говорит schema: $ref, но примеры не массивы? Оставляем один
    /// Фильтр по видимости контента.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub feed_visibility: Option<FeedVisibility>,
    /// Показывать ли цены офферов с периодичностью > месяца (для подписок).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_all_subscription_periods: Option<bool>,
}
