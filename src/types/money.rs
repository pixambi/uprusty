use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MoneyObject{
    pub currency_code: String,
    pub value: String,
    pub value_in_base_units: i64,
}