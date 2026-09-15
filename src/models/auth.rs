use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, uniffi::Record)]
pub struct AuthenticateQuery {
    pub username: String,
    pub password: String,
    pub ip: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, uniffi::Record)]
pub struct UserProfile {
    #[serde(rename = "displayName")]
    pub display_name: String,
    pub name: String,
    pub firstname: String,
    pub email: String,
    pub escn: Option<String>,
    pub roles: Vec<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, uniffi::Record)]
pub struct Authenticated {
    #[serde(rename = "displayName")]
    pub display_name: String,
    pub name: String,
    pub firstname: String,
    pub email: String,
    pub escn: Option<String>,
    pub roles: Vec<String>,
    #[serde(rename = "authToken")]
    pub auth_token: String,
    pub ip: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, uniffi::Record)]
pub struct GetUserResult {
    pub username: String,
    pub roles: Vec<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, uniffi::Record)]
pub struct SsoServiceTokenQuery {
    #[serde(rename = "authToken")]
    pub auth_token: String,
    pub service: String,
    pub ip: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, uniffi::Record)]
pub struct ReauthenticateQuery {
    pub uuid: String,
    pub key: String,
    pub iv: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, uniffi::Record)]
pub struct KeepAuthenticated {
    #[serde(rename = "displayName")]
    pub display_name: String,
    pub name: String,
    pub firstname: String,
    pub email: String,
    pub escn: Option<String>,
    pub roles: Vec<String>,
    #[serde(rename = "authToken")]
    pub auth_token: String,
    #[serde(rename = "refreshAuthToken")]
    pub refresh_auth_token: String,
    pub ip: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, uniffi::Record)]
pub struct LoginPageContentTranslation {
    #[serde(rename = "languagesCode", alias = "languages_code", default)]
    pub languages_code: Option<String>,
    #[serde(rename = "connectionText", alias = "connexion_text", default)]
    pub connection_text: Option<String>,
    #[serde(rename = "notAuthenticatedText", alias = "not_authenticated_text", default)]
    pub not_authenticated_text: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, uniffi::Record)]
pub struct LoginPageContentResult {
    pub translations: Vec<LoginPageContentTranslation>,
}
