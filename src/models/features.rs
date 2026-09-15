use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, uniffi::Record)]
pub struct Authorization {
    #[serde(rename = "type", alias = "auth_type", default)]
    pub auth_type: Option<String>,
    #[serde(default)]
    pub roles: Vec<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, uniffi::Record)]
pub struct SettingsByRole {
    #[serde(default)]
    pub role: String,
    #[serde(default)]
    pub position: i32,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, uniffi::Record)]
pub struct FeatureTranslation {
    #[serde(rename = "languagesCode", alias = "languages_code", default)]
    pub languages_code: Option<String>,
    pub title: Option<String>,
    #[serde(rename = "shortTitle", alias = "short_title", default)]
    pub short_title: Option<String>,
    #[serde(rename = "searchKeywords", alias = "search_keywords", default)]
    pub search_keywords: Option<Vec<String>>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, uniffi::Record)]
pub struct Feature {
    pub id: String,
    #[serde(rename = "type", alias = "access_type", default)]
    pub access_type: Option<String>,
    pub icon: Option<String>,
    #[serde(rename = "iconSvgLight", alias = "iconSourceSvgLightTheme", default)]
    pub icon_svg_light: Option<String>,
    #[serde(rename = "iconSvgDark", alias = "iconSourceSvgDarkTheme", default)]
    pub icon_svg_dark: Option<String>,
    #[serde(default)]
    pub position: i32,
    #[serde(rename = "statisticName", alias = "statistic_name", default)]
    pub statistic_name: Option<String>,
    pub authorization: Option<Authorization>,
    #[serde(rename = "settingsByRole", alias = "settings_by_role", default)]
    pub settings_by_role: Option<Vec<SettingsByRole>>,
    pub menu: Option<String>,
    #[serde(default)]
    pub translations: Vec<FeatureTranslation>,
    #[serde(rename = "routerLink", alias = "router_link", default)]
    pub router_link: Option<String>,
    pub link: Option<String>,
    #[serde(rename = "ssoService", alias = "sso_service", default)]
    pub sso_service: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, uniffi::Record)]
pub struct WidgetTranslation {
    #[serde(rename = "languagesCode", alias = "languages_code", default)]
    pub languages_code: Option<String>,
    pub title: Option<String>,
    pub content: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, uniffi::Record)]
pub struct Widget {
    pub id: String,
    #[serde(rename = "type", alias = "access_type", default)]
    pub access_type: Option<String>,
    pub icon: Option<String>,
    #[serde(rename = "iconSvgLight", alias = "iconSourceSvgLightTheme", default)]
    pub icon_svg_light: Option<String>,
    #[serde(rename = "iconSvgDark", alias = "iconSourceSvgDarkTheme", default)]
    pub icon_svg_dark: Option<String>,
    #[serde(default)]
    pub position: i32,
    #[serde(rename = "statisticName", alias = "statistic_name", default)]
    pub statistic_name: Option<String>,
    pub authorization: Option<Authorization>,
    #[serde(rename = "settingsByRole", alias = "settings_by_role", default)]
    pub settings_by_role: Option<Vec<SettingsByRole>>,
    pub widget: Option<String>,
    pub color: Option<String>,
    #[serde(default)]
    pub translations: Vec<WidgetTranslation>,
    #[serde(rename = "routerLink", alias = "router_link", default)]
    pub router_link: Option<String>,
    pub link: Option<String>,
    #[serde(rename = "ssoService", alias = "sso_service", default)]
    pub sso_service: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, uniffi::Record)]
pub struct ContentQueryResponse {
    #[serde(default)]
    pub features: Vec<Feature>,
    #[serde(default)]
    pub widgets: Vec<Widget>,
}
