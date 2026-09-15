use std::collections::HashMap;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, uniffi::Record)]
pub struct GpsCoordinate {
    #[serde(default)]
    pub lat: f64,
    #[serde(default)]
    pub lng: f64,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, uniffi::Record)]
pub struct Campus {
    pub id: String,
    #[serde(default)]
    pub name: String,
    pub photo: Option<String>,
    pub initial: Option<GpsCoordinate>,
    pub southwest: Option<GpsCoordinate>,
    pub northeast: Option<GpsCoordinate>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, uniffi::Record)]
pub struct MapIcon {
    pub id: Option<String>,
    pub svg: Option<String>,
    #[serde(default)]
    pub width: f64,
    #[serde(default)]
    pub height: f64,
    #[serde(default)]
    pub x: f64,
    #[serde(default)]
    pub y: f64,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, uniffi::Record)]
pub struct MapCategoryTranslation {
    #[serde(rename = "languagesCode", alias = "languages_code", default)]
    pub languages_code: Option<String>,
    #[serde(default)]
    pub label: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, uniffi::Record)]
pub struct MapCategory {
    pub id: String,
    pub translations: Option<Vec<MapCategoryTranslation>>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, uniffi::Record)]
pub struct MapPointTranslation {
    #[serde(rename = "languagesCode", alias = "languages_code", default)]
    pub languages_code: Option<String>,
    #[serde(default)]
    pub name: String,
    pub description: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, uniffi::Record)]
pub struct Marker {
    pub id: String,
    #[serde(default)]
    pub latitude: f64,
    #[serde(default)]
    pub longitude: f64,
    #[serde(rename = "campusId", alias = "campus_id")]
    pub campus_id: Option<String>,
    #[serde(rename = "iconId", alias = "icon_id")]
    pub icon_id: Option<String>,
    pub translations: Option<Vec<MapPointTranslation>>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, uniffi::Record)]
pub struct MapTranslation {
    pub value: Option<String>,
    #[serde(rename = "langcode", alias = "languagesCode", alias = "languages_code", default)]
    pub langcode: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, uniffi::Record)]
pub struct MapPointItem {
    pub category: Option<String>,
    pub title: Option<Vec<MapTranslation>>,
    pub description: Option<Vec<MapTranslation>>,
    #[serde(default)]
    pub latitude: f64,
    #[serde(default)]
    pub longitude: f64,
    pub icon: Option<MapIcon>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, uniffi::Record)]
pub struct MapData {
    #[serde(default)]
    pub icons: Vec<MapIcon>,
    #[serde(default)]
    pub categories: Vec<MapCategory>,
    #[serde(default)]
    pub campuses: Vec<Campus>,
    #[serde(rename = "markersCollections", alias = "markers_collections", default)]
    pub markers_collections: HashMap<String, Vec<Marker>>,
    #[serde(default)]
    pub items: Vec<MapPointItem>,
}

