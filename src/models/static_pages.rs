use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, uniffi::Record)]
pub struct StaticPageTranslation {
    #[serde(rename = "languagesCode", alias = "languages_code", default)]
    pub languages_code: Option<String>,
    #[serde(default)]
    pub title: String,
    #[serde(default)]
    pub content: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, uniffi::Record)]
pub struct StaticPage {
    pub id: i64,
    pub status: Option<String>,
    pub icon: Option<String>,
    #[serde(rename = "iconSvgLight", alias = "icon_svg_light", default)]
    pub icon_svg_light: Option<String>,
    #[serde(rename = "iconSvgDark", alias = "icon_svg_dark", default)]
    pub icon_svg_dark: Option<String>,
    pub translations: Option<Vec<StaticPageTranslation>>,
    #[serde(rename = "statisticName", alias = "statistic_name", default)]
    pub statistic_name: Option<String>,
    #[serde(alias = "sort", default)]
    pub position: Option<i32>,
}
