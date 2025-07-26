use crate::client::{Client, ClientError};
use crate::models::account::{AccountsResponse, AccountResponse, AccountType, OwnershipType};
use async_trait::async_trait;
use reqwest::Method;

#[async_trait]
pub trait AccountsExt{
    async fn list_accounts(&self, page_size: Option<u32>, account_type: Option<AccountType>, ownership_type: Option<OwnershipType>) -> Result<AccountsResponse, ClientError>;

    async fn get_account(&self, id: &str) -> Result<AccountResponse, ClientError>;
}

#[async_trait]
impl AccountsExt for Client{
    async fn list_accounts(&self, page_size: Option<u32>, account_type: Option<AccountType>, ownership_type: Option<OwnershipType>) -> Result<AccountsResponse, ClientError> {
        todo!()
    }
    async fn get_account(&self, id: &str) -> Result<AccountResponse, ClientError> {
        todo!()
    }

}