use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WebhookAttributes {
    pub url: String,
    pub description: Option<String>,
    pub secret_key: Option<String>,
    pub created_at: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct WebhookRelationships {
    pub logs: WebhookLogsRelationship,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct WebhookLogsRelationship {
    pub links: Option<WebhookRelationshipLinks>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct WebhookRelationshipLinks {
    pub related: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct WebhookLinks {
    #[serde(rename = "self")]
    pub self_link: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct WebhookResource {
    #[serde(rename = "type")]
    pub resource_type: String,
    pub id: String,
    pub attributes: WebhookAttributes,
    pub relationships: WebhookRelationships,
    pub links: Option<WebhookLinks>,
}

// Webhook response types
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct WebhooksResponse {
    pub data: Vec<WebhookResource>,
    pub links: PaginationLinks,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct WebhookResponse {
    pub data: WebhookResource,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PaginationLinks {
    pub prev: Option<String>,
    pub next: Option<String>,
}

// Webhook creation request
#[derive(Debug, Clone, Serialize)]
pub struct WebhookInputAttributes {
    pub url: String,
    pub description: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct WebhookInputResource {
    pub attributes: WebhookInputAttributes,
}

#[derive(Debug, Clone, Serialize)]
pub struct CreateWebhookRequest {
    pub data: WebhookInputResource,
}

impl CreateWebhookRequest {
    pub fn new(url: &str, description: Option<&str>) -> Self {
        Self {
            data: WebhookInputResource {
                attributes: WebhookInputAttributes {
                    url: url.to_string(),
                    description: description.map(|s| s.to_string()),
                },
            },
        }
    }
}

// Webhook events
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum WebhookEventType {
    TransactionCreated,
    TransactionSettled,
    TransactionDeleted,
    Ping,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WebhookEventAttributes {
    pub event_type: WebhookEventType,
    pub created_at: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct WebhookEventRelationships {
    pub webhook: WebhookEventWebhookRelationship,
    pub transaction: Option<WebhookEventTransactionRelationship>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct WebhookEventWebhookRelationship {
    pub data: ResourceIdentifier,
    pub links: Option<WebhookEventRelationshipLinks>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct WebhookEventTransactionRelationship {
    pub data: ResourceIdentifier,
    pub links: Option<WebhookEventRelationshipLinks>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ResourceIdentifier {
    #[serde(rename = "type")]
    pub resource_type: String,
    pub id: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct WebhookEventRelationshipLinks {
    pub related: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct WebhookEventResource {
    #[serde(rename = "type")]
    pub resource_type: String,
    pub id: String,
    pub attributes: WebhookEventAttributes,
    pub relationships: WebhookEventRelationships,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct WebhookEventResponse {
    pub data: WebhookEventResource,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum WebhookDeliveryStatus {
    Delivered,
    Undeliverable,
    BadResponseCode,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct WebhookDeliveryRequest {
    pub body: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WebhookDeliveryResponse {
    pub status_code: u16,
    pub body: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WebhookDeliveryLogAttributes {
    pub request: WebhookDeliveryRequest,
    pub response: Option<WebhookDeliveryResponse>,
    pub delivery_status: WebhookDeliveryStatus,
    pub created_at: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct WebhookDeliveryLogRelationships {
    #[serde(rename = "webhookEvent")]
    pub webhook_event: WebhookEventRelationship,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct WebhookEventRelationship {
    pub data: ResourceIdentifier,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct WebhookDeliveryLogResource {
    #[serde(rename = "type")]
    pub resource_type: String,
    pub id: String,
    pub attributes: WebhookDeliveryLogAttributes,
    pub relationships: WebhookDeliveryLogRelationships,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct WebhookDeliveryLogsResponse {
    pub data: Vec<WebhookDeliveryLogResource>,
    pub links: PaginationLinks,
}