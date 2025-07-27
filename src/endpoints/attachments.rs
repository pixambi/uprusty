use crate::client::{Client, ClientError};
use crate::models::attachment::{AttachmentResponse, AttachmentsResponse};
use async_trait::async_trait;
use reqwest::Method;

#[async_trait]
pub trait AttachmentsExt {
    /// Retrieve a list of all attachments. The returned list is paginated and can be scrolled
    /// by following the next and prev links where present.
    async fn list_attachments(&self, page_size: Option<u32>) -> Result<AttachmentsResponse, ClientError>;

    /// Retrieve a specific attachment by providing its unique identifier.
    async fn get_attachment(&self, id: &str) -> Result<AttachmentResponse, ClientError>;
}

#[async_trait]
impl AttachmentsExt for Client {
    async fn list_attachments(&self, page_size: Option<u32>) -> Result<AttachmentsResponse, ClientError> {
        let mut url = self.base_url.join("attachments")?;

        if let Some(size) = page_size {
            let mut query = url.query_pairs_mut();
            query.append_pair("page[size]", &size.to_string());
        }

        let response = self.request(Method::GET, url)?.send().await?;

        if response.status().is_success() {
            let attachments = response.json::<AttachmentsResponse>().await?;
            Ok(attachments)
        } else {
            match response.error_for_status() {
                Err(err) => Err(ClientError::RequestError(err)),
                Ok(_) => unreachable!("Expected an error due to non-success status code"),
            }
        }
    }

    async fn get_attachment(&self, id: &str) -> Result<AttachmentResponse, ClientError> {
        let url = self.base_url.join(&format!("attachments/{}", id))?;

        let response = self.request(Method::GET, url)?.send().await?;

        if response.status().is_success() {
            let attachment = response.json::<AttachmentResponse>().await?;
            Ok(attachment)
        } else {
            Err(ClientError::RequestError(
                response.error_for_status().unwrap_err(),
            ))
        }
    }
}