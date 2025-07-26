mod client;
mod types;
mod endpoints;
mod models;
mod error;

pub use client::Client;
pub use error::{Error, Result};

pub use endpoints::accounts::AccountsExt;

// Re-export important types
pub use models::account::{
    AccountResource,
    AccountsResponse,
    AccountResponse,
    AccountType,
    OwnershipType,
    AccountAttributes,
};

pub use types::money::MoneyObject;

pub mod prelude {
    pub use crate::client::Client;
    pub use crate::endpoints::accounts::AccountsExt;
    pub use crate::types::money::MoneyObject;
    pub use crate::models::account::*;
}
