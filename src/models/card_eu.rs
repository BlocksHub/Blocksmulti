use serde::{Deserialize, Serialize};
use crate::models::card::QrCode;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, uniffi::Record)]
pub struct UserCardEu {
    pub lastname: String,
    pub firstname: String,
    pub photo: String,
    pub euid: String,
    #[serde(rename = "qrCode")]
    pub qr_code: Option<QrCode>,
    pub errors: Option<Vec<String>>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, uniffi::Record)]
pub struct UserCardEuLight {
    pub fullname: String,
    pub euid: String,
    #[serde(rename = "qrCode")]
    pub qr_code: Option<QrCode>,
    pub errors: Option<Vec<String>>,
}
