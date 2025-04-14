use crate::models::common::*;
use crate::models::product::RecurrentDto; // Импорт из product.rs
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

// --- Ответы (для устаревшего /api/v1/feed) ---

/// Описание оффера в ответе устаревшего API /api/v1/feed.
#[derive(Deserialize, Debug, Clone)]
pub struct OfferDto {
    pub id: Uuid,
    pub name: Option<String>,
    pub description: Option<String>,
    /// Список цен.
    pub prices: Vec<PriceDto>, // PriceDto определен в common.rs
    #[deprecated(note = "Используйте поле periodicity в PriceDto")]
    pub recurrent: Option<RecurrentDto>,
    /// Список ID постов, доступных при покупке этого оффера.
    #[serde(rename = "availablePosts", default)]
    pub available_posts: Vec<Uuid>,
    /// Возможность удалить оффер.
    #[serde(rename = "canBeDeleted")]
    pub can_be_deleted: Option<bool>,
    /// Причины, по которым оффер нельзя удалить.
    #[serde(default)]
    pub reasons: Vec<DeleteNotAllowedReason>,
}

/// Описание продукта в ответе устаревшего API /api/v1/feed.
#[derive(Deserialize, Debug, Clone)]
pub struct ProductItemDto {
    pub id: Uuid,
    #[serde(rename = "accountId")]
    pub account_id: Option<Uuid>,
    pub title: Option<String>,
    #[serde(rename = "updatedAt")]
    pub updated_at: Option<DateTime<Utc>>,
    pub status: Option<Status>,
    #[serde(rename = "moderationStatus")]
    pub moderation_status: Option<ModerationStatus>,
    #[serde(rename = "type")]
    pub product_type: ProductType,
    #[serde(default)]
    pub offers: Vec<OfferDto>,
}

/// Элемент ленты в ответе устаревшего API /api/v1/feed.
#[derive(Deserialize, Debug, Clone)]
pub struct FeedItem {
    #[serde(rename = "type")]
    pub item_type: FeedItemType,
    /// В OpenAPI `data` ссылается только на `ProductItemDto`.
    pub data: Option<ProductItemDto>,
}

/// Пагинированный ответ для устаревшего API /api/v1/feed.
#[derive(Deserialize, Debug, Clone)]
pub struct FeedPageResponse {
    pub items: Vec<FeedItem>,
    pub page: Option<i64>, // В OpenAPI `number`, используем i64
    pub size: Option<i64>,
    pub total: Option<i64>,
}

// --- Параметры запросов (Query) ---

/// Параметры для запроса устаревшего API /api/v1/feed.
#[derive(Serialize, Debug, Default, Clone)]
#[serde(rename_all = "camelCase")]
pub struct GetFeedParams {
    /// Фильтр по типу контента.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_categories: Option<FeedItemType>, // В OpenAPI схема ссылается на enum, не массив
    /// Фильтр по типу продукта.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_types: Option<ProductType>, // В OpenAPI схема ссылается на enum, не массив
    /// Номер страницы.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i64>,
    /// Количество элементов на странице.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<i64>,
}
