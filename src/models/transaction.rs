use crate::types::money::MoneyObject;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum TransactionStatus {
    Held,
    Settled,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum CardPurchaseMethod {
    BarCode,
    Ocr,
    CardPin,
    CardDetails,
    CardOnFile,
    Ecommerce,
    MagneticStripe,
    Contactless,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TransactionAttributes {
    pub status: TransactionStatus,
    pub raw_text: Option<String>,
    pub description: String,
    pub message: Option<String>,
    pub is_categorizable: bool,
    pub hold_info: Option<HoldInfo>,
    pub round_up: Option<RoundUp>,
    pub cashback: Option<Cashback>,
    pub amount: MoneyObject,
    pub foreign_amount: Option<MoneyObject>,
    pub card_purchase_method: Option<CardPurchaseMethodInfo>,
    pub settled_at: Option<String>,
    pub created_at: String,
    pub transaction_type: Option<String>,
    pub note: Option<Note>,
    pub performing_customer: Option<Customer>,
    #[serde(rename = "deepLinkURL")]
    pub deep_link_url: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct HoldInfo {
    pub amount: MoneyObject,
    pub foreign_amount: Option<MoneyObject>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RoundUp {
    pub amount: MoneyObject,
    pub boost_portion: Option<MoneyObject>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Cashback {
    pub description: String,
    pub amount: MoneyObject,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CardPurchaseMethodInfo {
    pub method: CardPurchaseMethod,
    pub card_number_suffix: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Note {
    pub text: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Customer {
    pub display_name: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TransactionRelationships {
    pub account: AccountRelationship,
    #[serde(rename = "transferAccount")]
    pub transfer_account: TransferAccountRelationship,
    pub category: CategoryRelationship,
    #[serde(rename = "parentCategory")]
    pub parent_category: ParentCategoryRelationship,
    pub tags: TagsRelationship,
    pub attachment: AttachmentRelationship,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AccountRelationship {
    pub data: ResourceIdentifier,
    pub links: Option<RelationshipLinks>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TransferAccountRelationship {
    pub data: Option<ResourceIdentifier>,
    pub links: Option<RelationshipLinks>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CategoryRelationship {
    pub data: Option<ResourceIdentifier>,
    pub links: Option<CategoryRelationshipLinks>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ParentCategoryRelationship {
    pub data: Option<ResourceIdentifier>,
    pub links: Option<RelationshipLinks>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TagsRelationship {
    pub data: Vec<TagResourceIdentifier>,
    pub links: Option<TagsRelationshipLinks>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AttachmentRelationship {
    pub data: Option<ResourceIdentifier>,
    pub links: Option<RelationshipLinks>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ResourceIdentifier {
    #[serde(rename = "type")]
    pub resource_type: String,
    pub id: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TagResourceIdentifier {
    #[serde(rename = "type")]
    pub resource_type: String,
    pub id: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RelationshipLinks {
    pub related: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CategoryRelationshipLinks {
    #[serde(rename = "self")]
    pub self_link: String,
    pub related: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TagsRelationshipLinks {
    #[serde(rename = "self")]
    pub self_link: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TransactionLinks {
    #[serde(rename = "self")]
    pub self_link: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TransactionResource {
    #[serde(rename = "type")]
    pub resource_type: String,
    pub id: String,
    pub attributes: TransactionAttributes,
    pub relationships: TransactionRelationships,
    pub links: Option<TransactionLinks>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TransactionsResponse {
    pub data: Vec<TransactionResource>,
    pub links: PaginationLinks,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TransactionResponse {
    pub data: TransactionResource,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PaginationLinks {
    pub prev: Option<String>,
    pub next: Option<String>,
}
