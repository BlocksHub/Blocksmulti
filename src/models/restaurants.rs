use std::collections::HashMap;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, uniffi::Record)]
pub struct RestaurantOpening {
    #[serde(default)]
    pub label: String,
    #[serde(rename = "isOpen", alias = "is_open", default)]
    pub is_open: bool,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, uniffi::Record)]
pub struct Restaurant {
    pub id: i64,
    #[serde(default)]
    pub title: String,
    #[serde(rename = "shortDesc", alias = "short_desc", default)]
    pub short_desc: Option<String>,
    #[serde(default)]
    pub opening: Option<HashMap<String, RestaurantOpening>>,
    #[serde(default)]
    pub latitude: Option<f64>,
    #[serde(default)]
    pub longitude: Option<f64>,
    #[serde(rename = "thumbnailUrl", alias = "thumbnail_url", default)]
    pub thumbnail_url: Option<String>,
    #[serde(default)]
    pub contact: Option<String>,
    #[serde(default)]
    pub infos: Option<String>,
    #[serde(default)]
    pub zone: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, uniffi::Record)]
pub struct FoodCategory {
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub dishes: Vec<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, uniffi::Record)]
pub struct Meal {
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub foodcategory: Vec<FoodCategory>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, uniffi::Record)]
pub struct RestaurantMenu {
    #[serde(default)]
    pub date: String,
    #[serde(default)]
    pub meal: Vec<Meal>,
}
