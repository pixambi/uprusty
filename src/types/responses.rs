use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct ErrorResponse {
    pub errors: Vec<ErrorObject>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ErrorObject {
    pub status: String,
    pub title: String,
    pub detail: String,
    pub source: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ErrorSource {
    pub parameter: Option<String>,
    pub pointer: Option<String>,
}