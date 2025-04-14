use serde::Deserialize;
use url::Url;

/// Ответ с ссылкой на страницу доната.
#[derive(Deserialize, Debug, Clone)]
pub struct DonateResponse {
    /// URL страницы доната.
    pub url: Url,
}
