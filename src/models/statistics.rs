use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, uniffi::Record)]
pub struct StatisticsUserActionPayload {
    pub duid: String,
    pub action: String,
    pub functionality: String,
    pub platform: String,
    #[serde(rename = "connectionType", alias = "connection_type", default)]
    pub connection_type: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, uniffi::Record)]
pub struct StatisticsUserAction {
    pub uid: Option<String>,
    #[serde(rename = "userAgent", alias = "user_agent", default)]
    pub user_agent: Option<String>,
    #[serde(rename = "xForwardedFor", alias = "x_forwarded_for", default)]
    pub x_forwarded_for: Option<String>,
    pub duid: String,
    pub action: String,
    pub functionality: String,
    pub platform: String,
    #[serde(rename = "connectionType", alias = "connection_type", default)]
    pub connection_type: String,
}
