use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AttachmentAttributes {
    pub created_at: Option<String>,
    #[serde(rename = "fileURL")]
    pub file_url: Option<String>,
    #[serde(rename = "fileURLExpiresAt")]
    pub file_url_expires_at: String,
    pub file_extension: Option<String>,
    pub file_content_type: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AttachmentTransactionData {
    #[serde(rename = "type")]
    pub resource_type: String,
    pub id: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AttachmentTransactionRelationship {
    pub data: AttachmentTransactionData,
    pub links: Option<AttachmentRelationshipLinks>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AttachmentRelationshipLinks {
    pub related: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AttachmentRelationships {
    pub transaction: AttachmentTransactionRelationship,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AttachmentLinks {
    #[serde(rename = "self")]
    pub self_link: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AttachmentResource {
    #[serde(rename = "type")]
    pub resource_type: String,
    pub id: String,
    pub attributes: AttachmentAttributes,
    pub relationships: AttachmentRelationships,
    pub links: Option<AttachmentLinks>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AttachmentsResponse {
    pub data: Vec<AttachmentResource>,
    pub links: PaginationLinks,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AttachmentResponse {
    pub data: AttachmentResource,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PaginationLinks {
    pub prev: Option<String>,
    pub next: Option<String>,
}