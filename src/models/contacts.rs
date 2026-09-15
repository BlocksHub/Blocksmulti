use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, uniffi::Record)]
pub struct Contact {
    pub name: String,
    pub firstname: String,
    #[serde(rename = "phoneNumbers")]
    pub phone_numbers: Vec<String>,
    #[serde(rename = "mobileNumbers")]
    pub mobile_numbers: Vec<String>,
    #[serde(rename = "mailAddresses")]
    pub mail_addresses: Vec<String>,
    pub assignments: Vec<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, uniffi::Record)]
pub struct ContactQuery {
    #[serde(rename = "type")]
    pub query_type: String,
    pub value: String,
    #[serde(rename = "userId")]
    pub user_id: String,
}
