use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use url::Url;

// --- Enum Определения ---
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash, Default)]
#[serde(rename_all = "UPPERCASE")]
pub enum CurrencyDto {
    #[default]
    Rub,
    Usd,
    Eur,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash, Default)]
#[serde(rename_all = "UPPERCASE")]
pub enum LanguageDto {
    #[default]
    En,
    Ru,
    Es,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
#[serde(rename_all = "UPPERCASE")]
pub enum PaymentMethod {
    Bank131,
    Unlimint,
    Paypal,
    Stripe,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Periodicity {
    OneTime,
    Monthly,
    Period90Days,
    Period180Days,
    PeriodYear,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
#[serde(rename_all = "kebab-case")]
pub enum ContractStatusDto {
    New,
    // #[serde(rename = "in-progress")]
    InProgress,
    Completed,
    Failed,
    Cancelled,
    SubscriptionActive,
    SubscriptionExpired,
    SubscriptionCancelled,
    SubscriptionFailed,
    #[serde(other)]
    Unknown,
}

impl ContractStatusDto {
    pub fn to_snake_case_string(&self) -> Result<String, serde_json::Error> {
        let json_string = serde_json::to_string(&self)?;
        Ok(json_string.trim_matches('"').to_string())
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum InvoiceStatus {
    New,
    InProgress,
    Completed,
    Failed,
    #[serde(other)]
    Unknown,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum InvoiceType {
    OneTime,
    Recurring,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum SubscriptionStatus {
    Active,
    Cancelled,
    Failed,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum FeedItemType {
    Post,
    Product,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ProductType {
    Course,
    DigitalProduct,
    Book,
    Guide,
    Subscription,
    Audio,
    Mods,
    Consultation,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash, Default)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum FeedVisibility {
    All,
    #[default]
    OnlyVisible,
    OnlyHidden,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum PostType {
    Lesson,
    Post,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Status {
    Published,
    #[serde(other)]
    Unknown,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ModerationStatus {
    New,
    Rejected,
    Approved,
    Blocked,
    #[serde(other)]
    Unknown,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum DeleteNotAllowedReason {
    HasSales,
    HasPosts,
}

// --- Общие Структуры ---

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PriceDto {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<f64>,
    pub currency: CurrencyDto,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub periodicity: Option<Periodicity>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct ClientUtmDto {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub utm_source: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub utm_medium: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub utm_campaign: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub utm_term: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub utm_content: Option<String>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct ErrorResponse {
    pub error: Option<String>,
    pub details: Option<serde_json::Value>,
    pub timestamp: Option<DateTime<Utc>>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct PagedResponse<T> {
    pub items: Vec<T>,
    pub total: i64,
    pub page: i64,
    pub size: i64,
    #[serde(rename = "totalPages")]
    pub total_pages: Option<i64>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct PagedResponseV2<T> {
    pub items: Vec<T>,
    #[serde(rename = "nextPage")]
    pub next_page: Option<Url>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct AmountTotalDto {
    pub currency: CurrencyDto,
    pub amount: f64,
}

#[derive(Deserialize, Debug, Clone)]
pub struct BuyerDto {
    pub email: String,
}

// --- Вспомогательные модули для сериализации дат в query параметрах ---

pub(crate) mod opt_chrono_datetime_as_iso8601 {
    use chrono::{DateTime, Utc};
    use serde::{Serialize, Serializer};

    pub fn serialize<S>(date: &Option<DateTime<Utc>>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match date {
            Some(d) => serializer.serialize_str(&d.to_rfc3339()),
            None => serializer.serialize_none(),
        }
    }
}

pub(crate) mod opt_chrono_naive_date_as_str {
    use chrono::NaiveDate;
    use serde::{Serialize, Serializer};
    const FORMAT: &str = "%Y-%m-%d";

    pub fn serialize<S>(date: &Option<NaiveDate>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match date {
            Some(d) => serializer.serialize_str(&d.format(FORMAT).to_string()),
            None => serializer.serialize_none(),
        }
    }
}
