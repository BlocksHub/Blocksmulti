use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, uniffi::Record)]
pub struct AppUpdateInfos {
    #[serde(rename = "storeVersion")]
    pub store_version: Option<String>,
    #[serde(rename = "minVersionRequired")]
    pub min_version_required: Option<String>,
    #[serde(rename = "playStoreUrl")]
    pub play_store_url: Option<String>,
    #[serde(rename = "appStoreUrl")]
    pub app_store_url: Option<String>,
}
