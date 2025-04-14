use crate::models::common::*;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Тип события вебхука.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
#[serde(rename_all = "snake_case")]
pub enum WebhookEventType {
    PaymentSuccess,
    PaymentFailed,
    SubscriptionRecurringPaymentSuccess,
    SubscriptionRecurringPaymentFailed,
    SubscriptionCancelled,
    #[serde(other)]
    Unknown,
}

/// Информация о продукте в теле вебхука.
#[derive(Deserialize, Debug, Clone)]
pub struct WebhookProduct {
    pub id: Uuid,
    pub title: Option<String>,
}

/// Информация о покупателе в теле вебхука.
#[derive(Deserialize, Debug, Clone)]
pub struct WebhookBuyer {
    pub email: String,
}

/// Тело вебхука о покупке/подписке.
#[derive(Deserialize, Debug, Clone)]
pub struct PurchaseWebhookLog {
    #[serde(rename = "eventType")]
    pub event_type: WebhookEventType,
    /// Информация о продукте (может отсутствовать при отмене подписки).
    pub product: Option<WebhookProduct>,
    /// Идентификатор контракта.
    #[serde(rename = "contractId")]
    pub contract_id: Uuid,
    /// Идентификатор родительского контракта (для рекуррентных платежей).
    #[serde(rename = "parentContractId")]
    pub parent_contract_id: Option<Uuid>,
    /// Информация о покупателе (может отсутствовать при отмене подписки).
    pub buyer: Option<WebhookBuyer>,
    /// Сумма операции (может отсутствовать при отмене подписки).
    pub amount: Option<f64>,
    /// Валюта операции (может отсутствовать при отмене подписки).
    pub currency: Option<CurrencyDto>,
    /// Статус операции (может отсутствовать при отмене подписки).
    pub status: Option<ContractStatusDto>,
    /// Временная метка операции (может отсутствовать при отмене подписки).
    pub timestamp: Option<DateTime<Utc>>,
    /// UTM-метки, связанные с покупкой.
    #[serde(rename = "clientUtm")]
    pub client_utm: Option<ClientUtmDto>,
    /// Сообщение об ошибке (если была).
    #[serde(rename = "errorMessage")]
    pub error_message: Option<String>,
    /// Дата отмены подписки (только для `subscription.cancelled`).
    #[serde(rename = "cancelledAt")]
    pub cancelled_at: Option<DateTime<Utc>>,
    /// Дата, когда подписка истечет после отмены (только для `subscription.cancelled`).
    #[serde(rename = "willExpireAt")]
    pub will_expire_at: Option<DateTime<Utc>>,
}
