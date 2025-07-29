use crate::client::{Client, ClientError};
use crate::models::tags::{TagsResponse, TagsTransactionRequest};
use async_trait::async_trait;
use reqwest::Method;

#[async_trait]
pub trait TagsExt {
    async fn list_tags(&self, page_size: Option<u32>) -> Result<TagsResponse, ClientError>;

    async fn add_tags_to_transaction(
        &self,
        transaction_id: &str,
        tag_ids: Vec<&str>,
    ) -> Result<(), ClientError>;

    async fn remove_tags_from_transaction(
        &self,
        transaction_id: &str,
        tag_ids: Vec<&str>,
    ) -> Result<(), ClientError>;
}

#[async_trait]
impl TagsExt for Client {
    async fn list_tags(&self, page_size: Option<u32>) -> Result<TagsResponse, ClientError> {
        let mut url = self.base_url.join("tags")?;

        if let Some(size) = page_size {
            let mut query = url.query_pairs_mut();
            query.append_pair("page[size]", &size.to_string());
        }

        let response = self.request(Method::GET, url)?.send().await?;
        let response = response
            .error_for_status()
            .map_err(ClientError::RequestError)?;
        let tags = response.json::<TagsResponse>().await?;
        Ok(tags)
    }

    async fn add_tags_to_transaction(
        &self,
        transaction_id: &str,
        tag_ids: Vec<&str>,
    ) -> Result<(), ClientError> {
        let url = self.base_url.join(&format!(
            "transactions/{}/relationships/tags",
            transaction_id
        ))?;

        let body = TagsTransactionRequest::new(tag_ids);

        let response = self.request(Method::POST, url)?.json(&body).send().await?;

        self.handle_no_content_response(response).await
    }

    async fn remove_tags_from_transaction(
        &self,
        transaction_id: &str,
        tag_ids: Vec<&str>,
    ) -> Result<(), ClientError> {
        let url = self.base_url.join(&format!(
            "transactions/{}/relationships/tags",
            transaction_id
        ))?;

        let body = TagsTransactionRequest::new(tag_ids);

        let response = self
            .request(Method::DELETE, url)?
            .json(&body)
            .send()
            .await?;

        self.handle_no_content_response(response).await
    }
}
