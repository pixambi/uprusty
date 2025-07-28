mod client;
mod endpoints;
mod error;
mod models;
mod types;
mod webhook;

pub use client::Client;
pub use error::{Error, Result};

pub use endpoints::accounts::AccountsExt;
pub use endpoints::attachments::AttachmentsExt;
pub use endpoints::categories::CategoriesExt;
pub use endpoints::tags::TagsExt;
pub use endpoints::transactions::{TransactionsExt, TransactionFilters};

pub use models::account::{
    AccountAttributes, AccountResource, AccountResponse, AccountType, AccountsResponse,
    OwnershipType,
};

pub use models::attachment::{
    AttachmentAttributes, AttachmentResource, AttachmentResponse, AttachmentsResponse,
    AttachmentTransactionData, AttachmentRelationships,
};

pub use models::category::{
    CategoryAttributes, CategoryResource, CategoryResponse, CategoriesResponse,
    CategoryRelationships, CategoryResourceIdentifier, CategorizeTransactionRequest,
};

pub use models::tags::{
    TagResource, TagsResponse, TagRelationships, TagsTransactionRequest,
};

pub use models::transaction::{
    TransactionResource, TransactionResponse, TransactionsResponse, TransactionStatus,
    TransactionAttributes, TransactionRelationships, CardPurchaseMethod,
    HoldInfo, RoundUp, Cashback, Note, Customer,
};

pub use types::money::MoneyObject;

pub mod prelude {
    pub use crate::client::Client;
    pub use crate::endpoints::accounts::AccountsExt;
    pub use crate::endpoints::attachments::AttachmentsExt;
    pub use crate::endpoints::categories::CategoriesExt;
    pub use crate::endpoints::tags::TagsExt;
    pub use crate::endpoints::transactions::{TransactionsExt, TransactionFilters};
    pub use crate::types::money::MoneyObject;
}