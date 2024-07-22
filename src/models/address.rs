use serde::{
    Serialize,
    Deserialize,
};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Address {
    pub country: Option<String>,
    pub zip_code: Option<String>,
    pub city: Option<String>,
    pub line_1: Option<String>,
    pub line_2: Option<String>,
}
