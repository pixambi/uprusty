use crate::client::{Client, ClientError};
use crate::models::category::{CategoriesResponse, CategoryResponse, CategorizeTransactionRequest};
use async_trait::async_trait;
use reqwest::Method;

#[async_trait]
pub trait CategoriesExt {

    async fn list_categories(&self, parent_filter: Option<&str>) -> Result<CategoriesResponse, ClientError>;

    async fn get_category(&self, id: &str) -> Result<CategoryResponse, ClientError>;

    async fn categorize_transaction(
        &self,
        transaction_id: &str,
        category_id: Option<&str>
    ) -> Result<(), ClientError>;
}

#[async_trait]
impl CategoriesExt for Client {
    async fn list_categories(&self, parent_filter: Option<&str>) -> Result<CategoriesResponse, ClientError> {
        let mut url = self.base_url.join("categories")?;

        if let Some(parent) = parent_filter {
            let mut query = url.query_pairs_mut();
            query.append_pair("filter[parent]", parent);
        }

        let response = self.request(Method::GET, url)?.send().await?;
        let response = response.error_for_status().map_err(ClientError::RequestError)?;
        let categories = response.json::<CategoriesResponse>().await?;
        Ok(categories)
    }

    async fn get_category(&self, id: &str) -> Result<CategoryResponse, ClientError> {
        let url = self.base_url.join(&format!("categories/{}", id))?;

        let response = self.request(Method::GET, url)?.send().await?;
        let response = response.error_for_status().map_err(ClientError::RequestError)?;
        let category = response.json::<CategoryResponse>().await?;
        Ok(category)
    }

    async fn categorize_transaction(
        &self,
        transaction_id: &str,
        category_id: Option<&str>
    ) -> Result<(), ClientError> {
        let url = self.base_url.join(&format!(
            "transactions/{}/relationships/category",
            transaction_id
        ))?;

        let body = match category_id {
            Some(id) => CategorizeTransactionRequest::new(id),
            None => CategorizeTransactionRequest::remove_category(),
        };

        let response = self.request(Method::PATCH, url)?
            .json(&body)
            .send()
            .await?;

        // Expect 204 No Content on success
        if response.status().as_u16() == 204 {
            Ok(())
        } else {
            Err(ClientError::RequestError(
                reqwest::Error::from(response.error_for_status().unwrap_err())
            ))
        }
    }
}