use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CategoryAttributes {
    pub name: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CategoryRelationships {
    pub parent: CategoryParentRelationship,
    pub children: CategoryChildrenRelationship,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CategoryParentRelationship {
    pub data: Option<CategoryResourceIdentifier>,
    pub links: Option<CategoryRelationshipLinks>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CategoryChildrenRelationship {
    pub data: Vec<CategoryResourceIdentifier>,
    pub links: Option<CategoryRelationshipLinks>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CategoryResourceIdentifier {
    #[serde(rename = "type")]
    pub resource_type: String,
    pub id: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CategoryRelationshipLinks {
    pub related: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CategoryLinks {
    #[serde(rename = "self")]
    pub self_link: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CategoryResource {
    #[serde(rename = "type")]
    pub resource_type: String,
    pub id: String,
    pub attributes: CategoryAttributes,
    pub relationships: CategoryRelationships,
    pub links: Option<CategoryLinks>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CategoriesResponse {
    pub data: Vec<CategoryResource>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CategoryResponse {
    pub data: CategoryResource,
}

// For PATCH requests to categorize transactions
#[derive(Debug, Clone, Serialize)]
pub struct CategoryInputResourceIdentifier {
    #[serde(rename = "type")]
    pub resource_type: String,
    pub id: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct CategorizeTransactionRequest {
    pub data: Option<CategoryInputResourceIdentifier>,
}

impl CategorizeTransactionRequest {
    pub fn new(category_id: &str) -> Self {
        Self {
            data: Some(CategoryInputResourceIdentifier {
                resource_type: "categories".to_string(),
                id: category_id.to_string(),
            }),
        }
    }

    pub fn remove_category() -> Self {
        Self { data: None }
    }
}