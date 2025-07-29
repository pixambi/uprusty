use crate::client::{Client, ClientError};
use crate::models::webhooks::{
    CreateWebhookRequest, WebhookDeliveryLogsResponse, WebhookEventResponse, WebhookResponse,
    WebhooksResponse,
};
use async_trait::async_trait;
use reqwest::Method;

#[async_trait]
pub trait WebhooksExt {
    async fn list_webhooks(&self, page_size: Option<u32>) -> Result<WebhooksResponse, ClientError>;

    async fn create_webhook(
        &self,
        url: &str,
        description: Option<&str>,
    ) -> Result<WebhookResponse, ClientError>;

    async fn get_webhook(&self, id: &str) -> Result<WebhookResponse, ClientError>;

    async fn delete_webhook(&self, id: &str) -> Result<(), ClientError>;

    async fn ping_webhook(&self, id: &str) -> Result<WebhookEventResponse, ClientError>;

    async fn list_webhook_logs(
        &self,
        webhook_id: &str,
        page_size: Option<u32>,
    ) -> Result<WebhookDeliveryLogsResponse, ClientError>;
}

#[async_trait]
impl WebhooksExt for Client {
    async fn list_webhooks(&self, page_size: Option<u32>) -> Result<WebhooksResponse, ClientError> {
        let mut url = self.base_url.join("webhooks")?;

        if let Some(size) = page_size {
            let mut query = url.query_pairs_mut();
            query.append_pair("page[size]", &size.to_string());
        }

        let response = self.request(Method::GET, url)?.send().await?;
        let response = response
            .error_for_status()
            .map_err(ClientError::RequestError)?;
        let webhooks = response.json::<WebhooksResponse>().await?;
        Ok(webhooks)
    }

    async fn create_webhook(
        &self,
        url: &str,
        description: Option<&str>,
    ) -> Result<WebhookResponse, ClientError> {
        let url_endpoint = self.base_url.join("webhooks")?;
        let body = CreateWebhookRequest::new(url, description);

        let response = self
            .request(Method::POST, url_endpoint)?
            .json(&body)
            .send()
            .await?;

        let response = response
            .error_for_status()
            .map_err(ClientError::RequestError)?;
        let webhook = response.json::<WebhookResponse>().await?;
        Ok(webhook)
    }

    async fn get_webhook(&self, id: &str) -> Result<WebhookResponse, ClientError> {
        let url = self.base_url.join(&format!("webhooks/{}", id))?;

        let response = self.request(Method::GET, url)?.send().await?;
        let response = response
            .error_for_status()
            .map_err(ClientError::RequestError)?;
        let webhook = response.json::<WebhookResponse>().await?;
        Ok(webhook)
    }

    async fn delete_webhook(&self, id: &str) -> Result<(), ClientError> {
        let url = self.base_url.join(&format!("webhooks/{}", id))?;

        let response = self.request(Method::DELETE, url)?.send().await?;
        self.handle_no_content_response(response).await
    }

    async fn ping_webhook(&self, id: &str) -> Result<WebhookEventResponse, ClientError> {
        let url = self.base_url.join(&format!("webhooks/{}/ping", id))?;

        let response = self
            .request(Method::POST, url)?
            .header("Content-Type", "application/json")
            .body("")
            .send()
            .await?;

        let response = response
            .error_for_status()
            .map_err(ClientError::RequestError)?;
        let event = response.json::<WebhookEventResponse>().await?;
        Ok(event)
    }

    async fn list_webhook_logs(
        &self,
        webhook_id: &str,
        page_size: Option<u32>,
    ) -> Result<WebhookDeliveryLogsResponse, ClientError> {
        let mut url = self
            .base_url
            .join(&format!("webhooks/{}/logs", webhook_id))?;

        if let Some(size) = page_size {
            let mut query = url.query_pairs_mut();
            query.append_pair("page[size]", &size.to_string());
        }

        let response = self.request(Method::GET, url)?.send().await?;
        let response = response
            .error_for_status()
            .map_err(ClientError::RequestError)?;
        let logs = response.json::<WebhookDeliveryLogsResponse>().await?;
        Ok(logs)
    }
}
