use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, uniffi::Record)]
pub struct UserContactData {
    pub username: String,
    #[serde(rename = "userAgent")]
    pub user_agent: String,
    pub platform: Option<String>,
    #[serde(rename = "appVersion")]
    pub app_version: Option<String>,
    #[serde(rename = "connectionType")]
    pub connection_type: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, uniffi::Record)]
pub struct SendMailQuery {
    #[serde(rename = "replyTo")]
    pub reply_to: String,
    pub subject: String,
    pub text: String,
    #[serde(rename = "userData")]
    pub user_data: UserContactData,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, uniffi::Record)]
pub struct ContactUsPageTranslation {
    #[serde(rename = "languagesCode", alias = "languages_code", default)]
    pub languages_code: Option<String>,
    pub title: String,
    pub content: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, uniffi::Record)]
pub struct ContactUsPageContentResult {
    pub icon: Option<String>,
    pub translations: Vec<ContactUsPageTranslation>,
}
