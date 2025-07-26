mod client;
mod endpoints;
mod error;
mod models;
mod types;

pub use client::Client;
pub use error::{Error, Result};

pub use endpoints::accounts::AccountsExt;

// Re-export important types
pub use models::account::{
    AccountAttributes, AccountResource, AccountResponse, AccountType, AccountsResponse,
    OwnershipType,
};

pub use types::money::MoneyObject;

pub mod prelude {
    pub use crate::client::Client;
    pub use crate::endpoints::accounts::AccountsExt;
    pub use crate::models::account::*;
    pub use crate::types::money::MoneyObject;
}
