use crate::types::money::MoneyObject;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum AccountType {
    Saver,
    Transactional,
    HomeLoan,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum OwnershipType {
    Individual,
    Joint,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AccountAttributes {
    pub display_name: String,
    pub account_type: AccountType,
    pub ownership_type: OwnershipType,
    pub balance: MoneyObject,
    pub created_at: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AccountRelationships {
    pub transactions: TransactionRelationship,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TransactionRelationship {
    pub links: Option<RelationshipLinks>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RelationshipLinks {
    pub related: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AccountLinks {
    #[serde(rename = "self")]
    pub self_link: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AccountResource {
    #[serde(rename = "type")]
    pub resource_type: String,
    pub id: String,
    pub attributes: AccountAttributes,
    pub relationships: AccountRelationships,
    pub links: Option<AccountLinks>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AccountsResponse {
    pub data: Vec<AccountResource>,
    pub links: PaginationLinks,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AccountResponse {
    pub data: AccountResource,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PaginationLinks {
    pub prev: Option<String>,
    pub next: Option<String>,
}
