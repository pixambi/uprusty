mod client;
mod endpoints;
mod error;
mod models;
mod types;

pub use client::Client;
pub use error::{Error, Result};

pub use endpoints::accounts::AccountsExt;
pub use endpoints::attachments::AttachmentsExt;
pub use endpoints::categories::CategoriesExt;

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

pub use types::money::MoneyObject;

pub mod prelude {
    pub use crate::client::Client;
    pub use crate::endpoints::accounts::AccountsExt;
    pub use crate::endpoints::attachments::AttachmentsExt;
    pub use crate::endpoints::categories::CategoriesExt;
    pub use crate::models::attachment::*;
    pub use crate::models::category::*;
    pub use crate::types::money::MoneyObject;
}
