mod client;
mod endpoints;
mod error;
mod models;
mod types;
pub mod webhook;

pub use client::Client;
pub use error::{Error, Result};

pub use endpoints::accounts::AccountsExt;
pub use endpoints::attachments::AttachmentsExt;
pub use endpoints::categories::CategoriesExt;
pub use endpoints::tags::TagsExt;
pub use endpoints::transactions::{TransactionFilters, TransactionsExt};
pub use endpoints::webhooks::WebhooksExt;

pub use models::account::{
    AccountAttributes, AccountResource, AccountResponse, AccountType, AccountsResponse,
    OwnershipType,
};

pub use models::attachment::{
    AttachmentAttributes, AttachmentRelationships, AttachmentResource, AttachmentResponse,
    AttachmentTransactionData, AttachmentsResponse,
};

pub use models::category::{
    CategoriesResponse, CategorizeTransactionRequest, CategoryAttributes, CategoryRelationships,
    CategoryResource, CategoryResourceIdentifier, CategoryResponse,
};

pub use models::tags::{TagRelationships, TagResource, TagsResponse, TagsTransactionRequest};

pub use models::transaction::{
    CardPurchaseMethod, Cashback, Customer, HoldInfo, Note, RoundUp, TransactionAttributes,
    TransactionRelationships, TransactionResource, TransactionResponse, TransactionStatus,
    TransactionsResponse,
};

pub use models::webhooks::{
    CreateWebhookRequest, WebhookDeliveryLogResource, WebhookDeliveryLogsResponse,
    WebhookDeliveryStatus, WebhookEventResource, WebhookEventResponse, WebhookEventType,
    WebhookResource, WebhookResponse, WebhooksResponse,
};

pub use types::money::MoneyObject;

pub mod prelude {
    pub use crate::client::Client;
    pub use crate::endpoints::accounts::AccountsExt;
    pub use crate::endpoints::attachments::AttachmentsExt;
    pub use crate::endpoints::categories::CategoriesExt;
    pub use crate::endpoints::tags::TagsExt;
    pub use crate::endpoints::transactions::{TransactionFilters, TransactionsExt};
    pub use crate::endpoints::webhooks::WebhooksExt;
    pub use crate::types::money::MoneyObject;
    pub use crate::webhook::{WebhookEventHandler, WebhookHandler};
}
