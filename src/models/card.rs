use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, uniffi::Record)]
pub struct QrCode {
    #[serde(rename = "type")]
    pub qr_type: String,
    pub value: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, uniffi::Record)]
pub struct UserCard {
    pub lastname: String,
    pub firstname: String,
    pub birthdate: Option<String>,
    pub gender: Option<String>,
    pub title: String,
    pub subtitle: Option<String>,
    pub ine: Option<String>,
    pub csn: Option<String>,
    pub photo: String,
    pub affiliation: String,
    #[serde(rename = "idNumber")]
    pub id_number: String,
    #[serde(rename = "endDate")]
    pub end_date: i64,
    #[serde(rename = "qrCode")]
    pub qr_code: Option<QrCode>,
    pub errors: Option<Vec<String>>,
}
