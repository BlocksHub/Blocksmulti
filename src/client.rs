use std::collections::HashMap;
use std::sync::Arc;

use serde::de::DeserializeOwned;
use serde_json::json;
use url::Url;

use crate::{
    http::{endpoints::Endpoint, errors::HttpError, manager::HttpManager},
    models::{
        app_update::AppUpdateInfos,
        auth::{AuthenticateQuery, Authenticated, LoginPageContentResult},
        card::UserCard,
        card_eu::{UserCardEu, UserCardEuLight},
        clocking::ClockingReply,
        contact_us::ContactUsPageContentResult,
        contacts::Contact,
        features::{ContentQueryResponse, Feature, Widget},
        important_news::ImportantNews,
        mail_calendar::MailCalendarReply,
        map::MapData,
        notifications::{Channel, NotificationResult},
        restaurants::{Restaurant, RestaurantMenu},
        rss::FeedItem,
        schedule::Schedule,
        social_network::SocialNetwork,
        static_pages::StaticPage,
    },
};

#[derive(uniffi::Object)]
pub struct Client {
    pub(crate) http: HttpManager,
    pub(crate) auth_token: String,
    pub(crate) server_url: String,
}

#[uniffi::export(async_runtime = "tokio")]
impl Client {
    #[uniffi::constructor]
    pub fn new_unauthenticated(server_url: String) -> Result<Arc<Self>, HttpError> {
        let base_url = Url::parse(&server_url).map_err(HttpError::UrlParsing)?;
        let transport = HttpManager::transport()?;
        let http = HttpManager::from_token(transport, base_url, String::new());
        Ok(Arc::new(Self {
            auth_token: String::new(),
            server_url,
            http,
        }))
    }

    #[uniffi::constructor]
    pub fn from_token(server_url: String, auth_token: String) -> Result<Arc<Self>, HttpError> {
        let base_url = Url::parse(&server_url).map_err(HttpError::UrlParsing)?;
        let transport = HttpManager::transport()?;
        let http = HttpManager::from_token(transport, base_url, auth_token.clone());
        Ok(Arc::new(Self {
            auth_token,
            server_url,
            http,
        }))
    }

    #[uniffi::constructor]
    pub async fn login(
        server_url: String,
        username: String,
        password: String,
        ip: Option<String>,
    ) -> Result<Arc<Self>, HttpError> {
        let base_url = Url::parse(&server_url).map_err(HttpError::UrlParsing)?;
        let transport = HttpManager::transport()?;
        let query = AuthenticateQuery {
            username,
            password,
            ip,
        };
        let auth: Authenticated =
            HttpManager::post_anonymous(&transport, &base_url, Endpoint::Login, Some(query)).await?;

        let http = HttpManager::from_token(transport, base_url, auth.auth_token.clone());

        Ok(Arc::new(Self {
            auth_token: auth.auth_token,
            server_url,
            http,
        }))
    }

    pub async fn relogin(
        &self,
        username: String,
        password: String,
        ip: Option<String>,
    ) -> Result<Arc<Self>, HttpError> {
        Self::login(self.server_url.clone(), username, password, ip).await
    }

    pub fn auth_token(&self) -> String {
        self.auth_token.clone()
    }

    pub fn server_url(&self) -> String {
        self.server_url.clone()
    }

    pub async fn logout(&self) -> Result<(), HttpError> {
        let payload = json!({ "authToken": self.auth_token });
        let _: serde_json::Value = self.http.delete(Endpoint::Logout, Some(payload)).await?;
        Ok(())
    }

    pub async fn get_login_page_content(&self) -> Result<LoginPageContentResult, HttpError> {
        self.http.get(Endpoint::LoginPageContent).await
    }

    pub async fn get_features(&self) -> Result<ContentQueryResponse, HttpError> {
        let payload = self.auth_payload();
        let val: serde_json::Value = self.http.post(Endpoint::Features, Some(payload)).await?;

        if let Ok(resp) = serde_json::from_value::<ContentQueryResponse>(val.clone()) {
            Ok(resp)
        } else if let Ok(items) = serde_json::from_value::<Vec<serde_json::Value>>(val) {
            let mut features = Vec::new();
            let mut widgets = Vec::new();

            for item in items {
                let is_widget = item.get("widget").is_some()
                    || item
                        .get("id")
                        .and_then(|i| i.as_str())
                        .map_or(false, |s| s.starts_with("widget:"));

                if is_widget {
                    if let Ok(w) = serde_json::from_value::<Widget>(item) {
                        widgets.push(w);
                    }
                } else if let Ok(f) = serde_json::from_value::<Feature>(item) {
                    features.push(f);
                }
            }

            Ok(ContentQueryResponse { features, widgets })
        } else {
            Ok(ContentQueryResponse {
                features: vec![],
                widgets: vec![],
            })
        }
    }

    pub async fn get_card(&self) -> Result<UserCard, HttpError> {
        self.http.post(Endpoint::Card, Some(self.auth_payload())).await
    }

    pub async fn get_card_eu(&self) -> Result<UserCardEu, HttpError> {
        self.http.post(Endpoint::CardEu, Some(self.auth_payload())).await
    }

    pub async fn get_card_eu_light(&self) -> Result<UserCardEuLight, HttpError> {
        self.http.post(Endpoint::CardEuLight, Some(self.auth_payload())).await
    }

    pub async fn get_schedule(
        &self,
        start_date: String,
        end_date: String,
    ) -> Result<Schedule, HttpError> {
        let mut payload = self.auth_payload();
        payload["startDate"] = json!(start_date);
        payload["endDate"] = json!(end_date);
        self.http.post(Endpoint::Schedule, Some(payload)).await
    }

    pub async fn get_notifications(
        &self,
        offset: u32,
        length: u32,
    ) -> Result<Vec<NotificationResult>, HttpError> {
        let mut payload = self.auth_payload();
        payload["offset"] = json!(offset);
        payload["length"] = json!(length);
        self.http.post(Endpoint::Notifications, Some(payload)).await
    }

    pub async fn mark_notifications_read(
        &self,
        notification_ids: Vec<String>,
    ) -> Result<(), HttpError> {
        let mut payload = self.auth_payload();
        payload["notificationIds"] = json!(notification_ids);
        let _: serde_json::Value = self
            .http
            .post(Endpoint::NotificationsRead, Some(payload))
            .await?;
        Ok(())
    }

    pub async fn get_notification_channels(&self) -> Result<Vec<Channel>, HttpError> {
        self.http.get(Endpoint::Notifications).await
    }

    pub async fn register_fcm_token(
        &self,
        token: String,
        platform: String,
    ) -> Result<(), HttpError> {
        let mut payload = self.auth_payload();
        payload["token"] = json!(token);
        payload["platform"] = json!(platform);
        let _: serde_json::Value = self
            .http
            .post(Endpoint::NotificationsRegister, Some(payload))
            .await?;
        Ok(())
    }

    pub async fn unregister_fcm_token(&self, fcm_token: String) -> Result<(), HttpError> {
        let mut payload = self.auth_payload();
        payload["fcmToken"] = json!(fcm_token);
        let _: serde_json::Value = self
            .http
            .post(Endpoint::NotificationsUnregister, Some(payload))
            .await?;
        Ok(())
    }

    pub async fn get_clocking(&self) -> Result<ClockingReply, HttpError> {
        self.http.post(Endpoint::Clocking, Some(self.auth_payload())).await
    }

    pub async fn clock_in(&self) -> Result<ClockingReply, HttpError> {
        self.http.post(Endpoint::ClockIn, Some(self.auth_payload())).await
    }

    pub async fn get_restaurants(&self) -> Result<Vec<Restaurant>, HttpError> {
        self.http.get(Endpoint::Restaurants).await
    }

    pub async fn get_restaurant_menus(
        &self,
        id: String,
        date: String,
    ) -> Result<Vec<RestaurantMenu>, HttpError> {
        let val: serde_json::Value = self
            .http
            .get_query(Endpoint::RestaurantMenus, &[("id", &id), ("date", &date)])
            .await?;

        Ok(Self::parse_one_or_many(val))
    }

    pub async fn get_rss(&self) -> Result<Vec<FeedItem>, HttpError> {
        self.http.get(Endpoint::Rss).await
    }

    pub async fn get_social_network(&self) -> Result<Vec<SocialNetwork>, HttpError> {
        self.http.get(Endpoint::SocialNetwork).await
    }

    pub async fn get_static_pages(&self) -> Result<Vec<StaticPage>, HttpError> {
        self.http.get(Endpoint::StaticPages).await
    }

    pub async fn get_mail_calendar(&self) -> Result<MailCalendarReply, HttpError> {
        self.http.post(Endpoint::MailCalendar, Some(self.auth_payload())).await
    }

    pub async fn get_map(&self) -> Result<MapData, HttpError> {
        let val: serde_json::Value = self.http.get(Endpoint::Map).await?;
        if let Ok(map_data) = serde_json::from_value::<MapData>(val.clone()) {
            Ok(map_data)
        } else {
            Ok(MapData {
                icons: vec![],
                categories: vec![],
                campuses: vec![],
                markers_collections: HashMap::new(),
                items: Self::parse_one_or_many(val),
            })
        }
    }

    pub async fn get_important_news(&self) -> Result<Vec<ImportantNews>, HttpError> {
        self.http.post(Endpoint::ImportantNews, Some(self.auth_payload())).await
    }

    pub async fn contact_us(
        &self,
        from_email: String,
        subject: String,
        text: String,
    ) -> Result<(), HttpError> {
        let payload = json!({
            "userData": self.auth_payload(),
            "from": from_email,
            "subject": subject,
            "text": text
        });
        let _: serde_json::Value = self.http.post(Endpoint::ContactUs, Some(payload)).await?;
        Ok(())
    }

    pub async fn get_contact_us_page(&self) -> Result<ContactUsPageContentResult, HttpError> {
        self.http.get(Endpoint::ContactUs).await
    }

    pub async fn search_contacts(
        &self,
        search_type: String,
        value: String,
    ) -> Result<Vec<Contact>, HttpError> {
        let mut payload = self.auth_payload();
        payload["type"] = json!(search_type);
        payload["value"] = json!(value);
        self.http.post(Endpoint::Contacts, Some(payload)).await
    }

    pub async fn get_app_update_infos(&self) -> Result<AppUpdateInfos, HttpError> {
        self.http.get(Endpoint::AppUpdateInfos).await
    }

    pub async fn get_version(&self) -> Result<String, HttpError> {
        let res: serde_json::Value = self.http.get(Endpoint::Version).await?;
        Ok(res.get("version")
            .and_then(|v| v.as_str())
            .unwrap_or("unknown")
            .to_string())
    }

    pub async fn get_health(&self) -> Result<String, HttpError> {
        let res: serde_json::Value = self.http.get(Endpoint::Health).await?;
        Ok(res.get("message")
            .and_then(|v| v.as_str())
            .unwrap_or("unknown")
            .to_string())
    }

    pub async fn get_sso_service_token(&self, service: String) -> Result<String, HttpError> {
        let mut payload = self.auth_payload();
        payload["service"] = json!(service);
        let res_text = self
            .http
            .post_text(Endpoint::SsoServiceToken, Some(payload))
            .await?;

        let raw_trimmed = res_text.trim().trim_matches('"').to_string();

        if let Ok(val) = serde_json::from_str::<serde_json::Value>(&res_text) {
            if let Some(t) = val.get("serviceToken").and_then(|v| v.as_str()) {
                return Ok(t.to_string());
            }
            if let Some(t) = val.get("ticket").and_then(|v| v.as_str()) {
                return Ok(t.to_string());
            }
        }

        Ok(raw_trimmed)
    }

    pub async fn get_sso_authenticated_url(
        &self,
        service_url: String,
    ) -> Result<String, HttpError> {
        let ticket = self.get_sso_service_token(service_url.clone()).await?;
        let separator = if service_url.contains('?') { "&" } else { "?" };
        Ok(format!("{}{}ticket={}", service_url, separator, ticket))
    }

    pub async fn post_user_action_statistic(
        &self,
        duid: String,
        action: String,
        functionality: String,
        platform: String,
        connection_type: String,
    ) -> Result<(), HttpError> {
        let payload = json!({
            "authToken": if self.auth_token.is_empty() { None } else { Some(&self.auth_token) },
            "data": {
                "duid": duid,
                "action": action,
                "functionality": functionality,
                "platform": platform,
                "connectionType": connection_type
            }
        });
        let _ = self
            .http
            .post_text(Endpoint::StatisticsUserAction, Some(payload))
            .await?;
        Ok(())
    }
}

/// Internal helpers — NOT exported via UniFFI.
impl Client {
    fn auth_payload(&self) -> serde_json::Value {
        json!({ "authToken": self.auth_token })
    }

    fn parse_one_or_many<T: DeserializeOwned>(val: serde_json::Value) -> Vec<T> {
        serde_json::from_value::<Vec<T>>(val.clone())
            .unwrap_or_else(|_| {
                serde_json::from_value::<T>(val)
                    .map(|single| vec![single])
                    .unwrap_or_default()
            })
    }
}
