use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, uniffi::Record)]
pub struct NotificationResult {
    pub id: String,
    pub author: Option<String>,
    #[serde(default)]
    pub channel: String,
    pub color: Option<String>,
    pub icon: Option<String>,
    pub title: String,
    pub message: String,
    pub url: Option<String>,
    pub state: Option<String>,
    #[serde(rename = "creationDate", alias = "creation_date", default)]
    pub creation_date: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, uniffi::Record)]
pub struct ChannelTranslation {
    #[serde(rename = "languagesCode", alias = "languages_code", default)]
    pub languages_code: Option<String>,
    pub label: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, uniffi::Record)]
pub struct Channel {
    pub id: String,
    pub code: String,
    pub translations: Option<Vec<ChannelTranslation>>,
    pub icon: Option<String>,
    pub color: Option<String>,
    #[serde(rename = "routerLink")]
    pub router_link: Option<String>,
    pub filterable: bool,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, uniffi::Record)]
pub struct RegisterFcmTokenQuery {
    pub username: String,
    pub token: String,
    pub platform: String,
    pub ip: String,
}
