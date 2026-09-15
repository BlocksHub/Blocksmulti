use serde::{Deserialize, Serialize};
use crate::models::features::Authorization;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, uniffi::Record)]
pub struct ImportantNewsTranslation {
    #[serde(rename = "languagesCode", alias = "languages_code", default)]
    pub languages_code: Option<String>,
    #[serde(default)]
    pub title: String,
    #[serde(default)]
    pub content: String,
    #[serde(rename = "buttonLabel", alias = "button_label", default)]
    pub button_label: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, uniffi::Record)]
pub struct ImportantNews {
    pub id: i64,
    pub image: Option<String>,
    pub color: Option<String>,
    pub link: Option<String>,
    pub position: Option<i32>,
    pub authorization: Option<Authorization>,
    pub translations: Option<Vec<ImportantNewsTranslation>>,
    #[serde(rename = "statisticName", alias = "statistic_name")]
    pub statistic_name: Option<String>,
}
