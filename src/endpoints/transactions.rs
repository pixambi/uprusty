use crate::client::{Client, ClientError};
use crate::models::transaction::{TransactionResponse, TransactionStatus, TransactionsResponse};
use async_trait::async_trait;
use reqwest::Method;

#[derive(Debug, Clone)]
pub struct TransactionFilters {
    pub status: Option<TransactionStatus>,
    pub since: Option<String>,
    pub until: Option<String>,
    pub category: Option<String>,
    pub tag: Option<String>,
}

impl Default for TransactionFilters {
    fn default() -> Self {
        Self {
            status: None,
            since: None,
            until: None,
            category: None,
            tag: None,
        }
    }
}

#[async_trait]
pub trait TransactionsExt {
    async fn list_transactions(
        &self,
        page_size: Option<u32>,
        filters: Option<TransactionFilters>,
    ) -> Result<TransactionsResponse, ClientError>;

    async fn get_transaction(&self, id: &str) -> Result<TransactionResponse, ClientError>;

    async fn list_account_transactions(
        &self,
        account_id: &str,
        page_size: Option<u32>,
        filters: Option<TransactionFilters>,
    ) -> Result<TransactionsResponse, ClientError>;
}

#[async_trait]
impl TransactionsExt for Client {
    async fn list_transactions(
        &self,
        page_size: Option<u32>,
        filters: Option<TransactionFilters>,
    ) -> Result<TransactionsResponse, ClientError> {
        let mut url = self.base_url.join("transactions")?;

        {
            let mut query = url.query_pairs_mut();

            if let Some(size) = page_size {
                query.append_pair("page[size]", &size.to_string());
            }

            if let Some(filters) = filters {
                if let Some(status) = filters.status {
                    let status_str = match status {
                        TransactionStatus::Held => "HELD",
                        TransactionStatus::Settled => "SETTLED",
                    };
                    query.append_pair("filter[status]", status_str);
                }

                if let Some(since) = filters.since {
                    query.append_pair("filter[since]", &since);
                }

                if let Some(until) = filters.until {
                    query.append_pair("filter[until]", &until);
                }

                if let Some(category) = filters.category {
                    query.append_pair("filter[category]", &category);
                }

                if let Some(tag) = filters.tag {
                    query.append_pair("filter[tag]", &tag);
                }
            }
        }

        let response = self.request(Method::GET, url)?.send().await?;
        let response = response.error_for_status().map_err(ClientError::RequestError)?;
        let transactions = response.json::<TransactionsResponse>().await?;
        Ok(transactions)
    }

    async fn get_transaction(&self, id: &str) -> Result<TransactionResponse, ClientError> {
        let url = self.base_url.join(&format!("transactions/{}", id))?;

        let response = self.request(Method::GET, url)?.send().await?;
        let response = response.error_for_status().map_err(ClientError::RequestError)?;
        let transaction = response.json::<TransactionResponse>().await?;
        Ok(transaction)
    }

    async fn list_account_transactions(
        &self,
        account_id: &str,
        page_size: Option<u32>,
        filters: Option<TransactionFilters>,
    ) -> Result<TransactionsResponse, ClientError> {
        let mut url = self.base_url.join(&format!("accounts/{}/transactions", account_id))?;

        {
            let mut query = url.query_pairs_mut();

            if let Some(size) = page_size {
                query.append_pair("page[size]", &size.to_string());
            }

            if let Some(filters) = filters {
                if let Some(status) = filters.status {
                    let status_str = match status {
                        TransactionStatus::Held => "HELD",
                        TransactionStatus::Settled => "SETTLED",
                    };
                    query.append_pair("filter[status]", status_str);
                }

                if let Some(since) = filters.since {
                    query.append_pair("filter[since]", &since);
                }

                if let Some(until) = filters.until {
                    query.append_pair("filter[until]", &until);
                }

                if let Some(category) = filters.category {
                    query.append_pair("filter[category]", &category);
                }

                if let Some(tag) = filters.tag {
                    query.append_pair("filter[tag]", &tag);
                }
            }
        }

        let response = self.request(Method::GET, url)?.send().await?;
        let response = response.error_for_status().map_err(ClientError::RequestError)?;
        let transactions = response.json::<TransactionsResponse>().await?;
        Ok(transactions)
    }
}