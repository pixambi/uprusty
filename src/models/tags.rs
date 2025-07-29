use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TagRelationships {
    pub transactions: TagTransactionsRelationship,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TagTransactionsRelationship {
    pub links: Option<TagRelationshipLinks>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TagRelationshipLinks {
    pub related: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TagResource {
    #[serde(rename = "type")]
    pub resource_type: String,
    pub id: String,
    pub relationships: TagRelationships,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TagsResponse {
    pub data: Vec<TagResource>,
    pub links: PaginationLinks,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PaginationLinks {
    pub prev: Option<String>,
    pub next: Option<String>,
}

// For POST/DELETE requests to add/remove tags from transactions
#[derive(Debug, Clone, Serialize)]
pub struct TagInputResourceIdentifier {
    #[serde(rename = "type")]
    pub resource_type: String,
    pub id: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct TagsTransactionRequest {
    pub data: Vec<TagInputResourceIdentifier>,
}

impl TagsTransactionRequest {
    pub fn new(tag_ids: Vec<&str>) -> Self {
        Self {
            data: tag_ids
                .into_iter()
                .map(|id| TagInputResourceIdentifier {
                    resource_type: "tags".to_string(),
                    id: id.to_string(),
                })
                .collect(),
        }
    }
}
