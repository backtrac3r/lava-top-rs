use crate::models::common::*;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use url::Url;
use uuid::Uuid;

// --- Запросы ---

/// Параметры для создания счета (контракта) на покупку.
#[derive(Serialize, Debug, Clone, Default)]
pub struct InvoiceRequestDto {
    /// Почта покупателя.
    pub email: String,
    /// Идентификатор ценового предложения (оффера).
    #[serde(rename = "offerId")]
    pub offer_id: Uuid,
    /// Периодичность оплаты (для подписок). Если None и продукт - не подписка, используется ONE_TIME.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub periodicity: Option<Periodicity>,
    /// Валюта покупки.
    pub currency: CurrencyDto,
    /// Способ оплаты. Если None, используется значение по умолчанию (BANK131 для RUB, UNLIMINT для USD/EUR).
    #[serde(rename = "paymentMethod")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_method: Option<PaymentMethod>,
    /// Язык покупателя для нотификаций. По умолчанию EN.
    #[serde(rename = "buyerLanguage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub buyer_language: Option<LanguageDto>,
    /// UTM-метки покупки.
    #[serde(rename = "clientUtm")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_utm: Option<ClientUtmDto>,
}

/// Ответ при создании счета (v1 и v2). Содержит ссылку на оплату.
#[derive(Deserialize, Debug, Clone)]
pub struct InvoicePaymentParamsResponse {
    /// Идентификатор созданного контракта.
    pub id: Uuid,
    pub status: ContractStatusDto,
    /// Сумма к оплате.
    #[serde(rename = "amountTotal")]
    pub amount_total: AmountTotalDto,
    /// Ссылка на виджет оплаты (может быть null, если продукт бесплатный).
    #[serde(rename = "paymentUrl")]
    pub payment_url: Option<Url>,
}

// --- Структуры для ответа GET /api/v1/invoices/{id} и списка GET /api/v1/invoices ---

#[derive(Deserialize, Debug, Clone)]
pub struct InvoiceReceiptResponse {
    pub amount: f64,
    pub currency: CurrencyDto,
    pub fee: Option<f64>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct InvoiceBuyerResponse {
    pub email: String,
    #[serde(rename = "cardMask")]
    pub card_mask: Option<String>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct InvoiceProductResponse {
    pub name: Option<String>,
    pub offer: Option<String>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct InvoiceParentInvoiceResponse {
    pub id: Uuid,
}

#[derive(Deserialize, Debug, Clone)]
pub struct InvoiceSubscriptionDetails {
    #[serde(rename = "expiredAt")]
    pub expired_at: Option<DateTime<Utc>>,
    #[serde(rename = "terminatedAt")]
    pub terminated_at: Option<DateTime<Utc>>,
    #[serde(rename = "cancelledAt")]
    pub cancelled_at: Option<DateTime<Utc>>,
}

/// Полная информация о контракте (ответ v2).
#[derive(Deserialize, Debug, Clone)]
pub struct InvoiceResponseV2 {
    pub id: Uuid,
    #[serde(rename = "type")]
    pub invoice_type: InvoiceType,
    /// Время исполнения контракта (если пусто, то время создания).
    pub datetime: DateTime<Utc>,
    /// Статус контракта (использует InvoiceStatus).
    pub status: InvoiceStatus,
    pub receipt: Option<InvoiceReceiptResponse>,
    pub buyer: Option<InvoiceBuyerResponse>,
    pub product: Option<InvoiceProductResponse>,
    #[serde(rename = "parentInvoice")]
    pub parent_invoice: Option<InvoiceParentInvoiceResponse>,
    #[serde(rename = "subscriptionStatus")]
    pub subscription_status: Option<SubscriptionStatus>,
    #[serde(rename = "subscriptionDetails")]
    pub subscription_details: Option<InvoiceSubscriptionDetails>,
    #[serde(rename = "clientUtm")]
    pub client_utm: Option<ClientUtmDto>,
}

/// Структура для пагинированного ответа списка контрактов.
#[derive(Deserialize, Debug, Clone)]
pub struct InvoicePageResponse {
    pub items: Vec<InvoiceResponseV2>,
    pub page: i64,
    pub size: i64,
    pub total: i64,
}

// --- Параметры запросов (Query) ---

/// Параметры для запроса списка контрактов (GET /api/v1/invoices).
#[derive(Serialize, Debug, Default, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ListInvoicesParams {
    #[serde(
        skip_serializing_if = "Option::is_none",
        with = "opt_chrono_datetime_as_iso8601",
        default
    )]
    pub begin_date: Option<DateTime<Utc>>,
    #[serde(
        skip_serializing_if = "Option::is_none",
        with = "opt_chrono_datetime_as_iso8601",
        default
    )]
    pub end_date: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub buyer_email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currencies: Option<Vec<CurrencyDto>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last4_card_digits: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice_types: Option<Vec<InvoiceType>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice_statuses: Option<Vec<InvoiceStatus>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<i64>,
}
