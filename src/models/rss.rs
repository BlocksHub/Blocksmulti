use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, uniffi::Record)]
pub struct FeedItem {
    #[serde(default)]
    pub title: String,
    #[serde(default)]
    pub content: Option<String>,
    #[serde(default)]
    pub link: Option<String>,
    #[serde(rename = "pubDate", alias = "pub_date", default)]
    pub pub_date: Option<String>,
    #[serde(default)]
    pub guid: Option<String>,
}
