use serde::Serialize;
use uuid::Uuid;

/// Параметры для отмены подписки (DELETE /api/v1/subscriptions).
#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CancelSubscriptionParams {
    /// Идентификатор родительского контракта (полученный при первой покупке).
    #[serde(rename = "contractId")]
    pub contract_id: Uuid,
    /// Email пользователя, владеющего подпиской.
    pub email: String,
}
