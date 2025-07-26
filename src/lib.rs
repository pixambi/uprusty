
mod client;
mod types;
mod endpoints;
mod models;

pub use client::Client;
pub use error::{Error, Result};

pub use endpoints::{
    AccountsExt,
};

pub mod prelude {
    pub use crate::client::Client;
    pub use crate::endpoints::*;
}