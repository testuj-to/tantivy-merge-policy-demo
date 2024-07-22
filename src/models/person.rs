use serde::{
    Serialize,
    Deserialize,
};

use super::address::Address;

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Person {
    pub id: String,
    pub first_name: String,
    pub last_name: String,
    pub sex: String,
    pub email: String,
    pub address: Option<Address>,
    pub settings: Option<PersonSettings>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PersonSettings {
    pub locale: Option<String>,
}
