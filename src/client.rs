use crate::error::{ApiErrorDetails, LavaTopError};
use crate::models::common::PagedResponseV2;
use crate::models::donate::DonateResponse;
use crate::models::feed::{FeedPageResponse, GetFeedParams};
use crate::models::invoice::{
    InvoicePageResponse, InvoicePaymentParamsResponse, InvoiceRequestDto, InvoiceResponseV2,
    ListInvoicesParams,
};
use crate::models::product::{
    FeedItemCombined, ListProductsParams, ProductItemResponse, ProductUpdateRequest,
};
use crate::models::report::{
    ListPartnerProductSalesParams, ListPartnerSalesParams, PartnerProductSalesPageResponse,
    PartnerSalesPageResponse,
};
use crate::models::subscription::CancelSubscriptionParams;
use reqwest::header::{ACCEPT, CONTENT_TYPE, HeaderMap, HeaderValue};
use reqwest::{Client as ReqwestClient, Method, Response, StatusCode};
use serde::Serialize;
use serde::de::DeserializeOwned;
use std::time::Duration;
use url::Url;
use uuid::Uuid;

const API_KEY_HEADER: &str = "X-Api-Key";

/// Асинхронный клиент для взаимодействия с Lava Top API.
#[derive(Clone, Debug)]
pub struct LavaTopClient {
    client: ReqwestClient,
    api_key: String,
    base_url: Url,
}

impl LavaTopClient {
    /// Создает новый экземпляр клиента.
    ///
    /// # Arguments
    ///
    /// * `api_key` - Ваш API ключ (X-Api-Key).
    /// * `base_url` - Опциональный базовый URL API. По умолчанию используется "https://gate.lava.top/".
    pub fn new(api_key: String, base_url: Option<Url>) -> Result<Self, LavaTopError> {
        let base = base_url.unwrap_or_else(|| {
            Url::parse("https://gate.lava.top/")
                .expect("Неверный базовый URL по умолчанию. Это ошибка в библиотеке.")
        });

        // Проверяем, что API ключ не пустой
        if api_key.trim().is_empty() {
            return Err(LavaTopError::MissingParameter(
                "api_key не может быть пустым".to_string(),
            ));
        }

        Ok(LavaTopClient {
            client: ReqwestClient::builder()
                .timeout(Duration::from_secs(30))
                .build()?, // Преобразуем ошибку reqwest в LavaTopError
            api_key,
            base_url: base,
        })
    }

    /// Устанавливает новый базовый URL для клиента.
    pub fn set_base_url(&mut self, url: Url) {
        self.base_url = url;
    }

    /// Внутренний метод для построения полного URL эндпоинта.
    fn build_url(&self, endpoint: &str) -> Result<Url, LavaTopError> {
        self.base_url
            .join(endpoint.trim_start_matches('/'))
            .map_err(LavaTopError::UrlParse)
    }

    /// Внутренний метод для отправки запросов к API.
    async fn send_request<T: Serialize, P: Serialize>(
        &self,
        method: Method,
        endpoint: &str,
        query_params: Option<&P>,
        json_body: Option<&T>,
    ) -> Result<Response, LavaTopError> {
        let url = self.build_url(endpoint)?;
        let mut headers = HeaderMap::new();
        headers.insert(ACCEPT, HeaderValue::from_static("application/json"));
        headers.insert(API_KEY_HEADER, HeaderValue::from_str(&self.api_key)?);

        let mut request_builder = self.client.request(method, url.clone()); // Клонируем URL для дебага

        if let Some(params) = query_params {
            request_builder = request_builder.query(params);
        }

        if let Some(body) = json_body {
            // Добавляем Content-Type только если есть тело
            request_builder = request_builder.header(CONTENT_TYPE, "application/json");
            request_builder = request_builder.json(body);
        }

        // Добавляем заголовки ПОСЛЕ установки тела/параметров, чтобы они не были перезаписаны
        request_builder = request_builder.headers(headers);

        // Debug: Вывод URL и тела
        // if let Ok(req_info) = request_builder.try_clone().map(|rb| rb.build()).and_then(|r| r) {
        //     println!("Request URL: {}", req_info.url());
        //     if let Some(body_bytes) = req_info.body().and_then(|b| b.as_bytes()) {
        //          if let Ok(body_str) = std::str::from_utf8(body_bytes) {
        //              println!("Request Body: {}", body_str);
        //          }
        //     }
        // }

        let response = request_builder.send().await?;

        // Debug: Вывод статуса ответа
        // println!("Response Status: {}", response.status());

        Ok(response)
    }

    /// Внутренний метод для обработки ответа и десериализации JSON.
    async fn process_response<R: DeserializeOwned>(
        &self,
        response: Response,
    ) -> Result<R, LavaTopError> {
        let status = response.status();
        if status.is_success() {
            // Успешный ответ (2xx)
            // Пытаемся десериализовать тело
            match response.json::<R>().await {
                Ok(data) => Ok(data),
                Err(e) => {
                    // Если десериализация не удалась даже при успешном статусе
                    Err(LavaTopError::Reqwest(e))
                }
            }
        } else {
            // Ошибка API (не 2xx)
            let raw_body = response.text().await.ok();
            // Debug: Вывод тела ошибки
            // if let Some(ref body) = raw_body {
            //     println!("Response Body (Error): {}", body);
            // }
            // Пытаемся парсить как стандартную структуру ошибки API
            let details: Option<ApiErrorDetails> = raw_body
                .as_deref()
                .and_then(|body| serde_json::from_str(body).ok());

            Err(LavaTopError::ApiError {
                status,
                details,
                raw_body,
            })
        }
    }

    // --- Реализация методов API ---

    // == Feed (Deprecated) ==
    #[deprecated(note = "Эндпоинт /api/v1/feed устарел. Используйте list_products_v2.")]
    /// Получение ленты продуктов и постов (устаревший метод).
    pub async fn get_feed(
        &self,
        params: Option<&GetFeedParams>,
    ) -> Result<FeedPageResponse, LavaTopError> {
        let response = self
            .send_request::<(), _>(Method::GET, "/api/v1/feed", params, None)
            .await?;
        self.process_response(response).await
    }

    // == Invoices ==

    #[deprecated(note = "Эндпоинт POST /api/v1/invoice устарел. Используйте create_invoice_v2.")]
    /// Создание контракта на покупку контента (устаревший метод v1).
    pub async fn create_invoice_v1(
        &self,
        request: &InvoiceRequestDto,
    ) -> Result<InvoicePaymentParamsResponse, LavaTopError> {
        let response = self
            .send_request::<_, ()>(Method::POST, "/api/v1/invoice", None, Some(request))
            .await?;
        self.process_response(response).await
    }

    /// Создание контракта на покупку контента (v2).
    pub async fn create_invoice_v2(
        &self,
        request: &InvoiceRequestDto,
    ) -> Result<InvoicePaymentParamsResponse, LavaTopError> {
        let response = self
            .send_request::<_, ()>(Method::POST, "/api/v2/invoice", None, Some(request))
            .await?;
        self.process_response(response).await
    }

    /// Получение страницы контрактов API-ключа.
    pub async fn list_invoices(
        &self,
        params: Option<&ListInvoicesParams>,
    ) -> Result<InvoicePageResponse, LavaTopError> {
        let response = self
            .send_request::<(), _>(Method::GET, "/api/v1/invoices", params, None)
            .await?;
        self.process_response(response).await
    }

    /// Получение контракта по идентификатору.
    pub async fn get_invoice_by_id(
        &self,
        invoice_id: &Uuid,
    ) -> Result<InvoiceResponseV2, LavaTopError> {
        let endpoint = format!("/api/v1/invoices/{}", invoice_id);
        let response = self
            .send_request::<(), ()>(Method::GET, &endpoint, None, None)
            .await?;
        self.process_response(response).await
    }

    // == Sales (Reports) ==

    /// Получение списка продаж партнёра (общий).
    pub async fn list_partner_sales(
        &self,
        params: Option<&ListPartnerSalesParams>,
    ) -> Result<PartnerSalesPageResponse, LavaTopError> {
        let response = self
            .send_request::<(), _>(Method::GET, "/api/v1/sales/", params, None)
            .await?;
        self.process_response(response).await
    }

    /// Получение списка продаж партнёра по конкретному продукту.
    pub async fn list_partner_product_sales(
        &self,
        product_id: Uuid,
        params: Option<&ListPartnerProductSalesParams>,
    ) -> Result<PartnerProductSalesPageResponse, LavaTopError> {
        let endpoint = format!("/api/v1/sales/{}", product_id);
        let response = self
            .send_request::<(), _>(Method::GET, &endpoint, params, None)
            .await?;
        self.process_response(response).await
    }

    // == Subscriptions ==

    /// Отмена подписки на продукт.
    ///
    /// # Arguments
    /// * `params` - Параметры отмены, содержащие `contract_id` и `email`.
    pub async fn cancel_subscription(
        &self,
        params: &CancelSubscriptionParams,
    ) -> Result<(), LavaTopError> {
        let response = self
            .send_request::<(), _>(Method::DELETE, "/api/v1/subscriptions", Some(params), None)
            .await?;

        let status = response.status();
        if status == StatusCode::NO_CONTENT {
            Ok(())
        } else if status.is_success() {
            // Неожиданный успешный статус
            eprintln!(
                "Warning: cancel_subscription received unexpected success status: {}",
                status
            );
            Ok(())
        } else {
            // Обрабатываем ошибку как обычно
            let raw_body = response.text().await.ok();
            let details: Option<ApiErrorDetails> = raw_body
                .as_deref()
                .and_then(|body| serde_json::from_str(body).ok());
            Err(LavaTopError::ApiError {
                status,
                details,
                raw_body,
            })
        }
    }

    // == Products ==

    /// Получение списка продуктов (v2).
    pub async fn list_products_v2(
        &self,
        params: Option<&ListProductsParams>,
    ) -> Result<PagedResponseV2<FeedItemCombined>, LavaTopError> {
        let response = self
            .send_request::<(), _>(Method::GET, "/api/v2/products", params, None)
            .await?;
        self.process_response(response).await
    }

    /// Обновление продукта.
    pub async fn update_product(
        &self,
        product_id: Uuid,
        request: &ProductUpdateRequest,
    ) -> Result<ProductItemResponse, LavaTopError> {
        let endpoint = format!("/api/v2/products/{}", product_id);
        let response = self
            .send_request::<_, ()>(Method::PATCH, &endpoint, None, Some(request))
            .await?;
        self.process_response(response).await
    }

    // == Donate ==

    /// Получение ссылки на донат аккаунта.
    pub async fn get_donate_link(&self) -> Result<DonateResponse, LavaTopError> {
        let response = self
            .send_request::<(), ()>(Method::GET, "/api/v1/donate", None, None)
            .await?;
        self.process_response(response).await
    }
}
