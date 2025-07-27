use crate::client::{Client, ClientError};
use crate::models::account::{AccountResponse, AccountType, AccountsResponse, OwnershipType};
use async_trait::async_trait;
use reqwest::Method;

#[async_trait]
pub trait AccountsExt {
    async fn list_accounts(
        &self,
        page_size: Option<u32>,
        account_type: Option<AccountType>,
        ownership_type: Option<OwnershipType>,
    ) -> Result<AccountsResponse, ClientError>;

    async fn get_account(&self, id: &str) -> Result<AccountResponse, ClientError>;
}

#[async_trait]
impl AccountsExt for Client {
    async fn list_accounts(
        &self,
        page_size: Option<u32>,
        account_type: Option<AccountType>,
        ownership_type: Option<OwnershipType>,
    ) -> Result<AccountsResponse, ClientError> {
        let mut url = self.base_url.join("accounts")?;

        {
            let mut query = url.query_pairs_mut();

            if let Some(size) = page_size {
                query.append_pair("page[size]", &size.to_string());
            }

            if let Some(account_type) = account_type {
                let filter_value = match account_type {
                    AccountType::Saver => "SAVER",
                    AccountType::Transactional => "TRANSACTIONAL",
                    AccountType::HomeLoan => "HOME_LOAN",
                };
                query.append_pair("filter[accountType]", filter_value);
            }

            if let Some(ownership_type) = ownership_type {
                let filter_value = match ownership_type {
                    OwnershipType::Individual => "INDIVIDUAL",
                    OwnershipType::Joint => "JOINT",
                };
                query.append_pair("filter[ownershipType]", filter_value);
            }
        }

        let response = self.request(Method::GET, url)?.send().await?;
        let response = response.error_for_status().map_err(ClientError::RequestError)?;
        let accounts = response.json::<AccountsResponse>().await?;
        Ok(accounts)
    }

    async fn get_account(&self, id: &str) -> Result<AccountResponse, ClientError> {
        let url = self.base_url.join(&format!("accounts/{}", id))?;

        let response = self.request(Method::GET, url)?.send().await?;
        let response = response.error_for_status().map_err(ClientError::RequestError)?;
        let account = response.json::<AccountResponse>().await?;
        Ok(account)
    }
}