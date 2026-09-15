use url::Url;

use crate::http::errors::HttpError;

pub enum Endpoint {
    Login,
    Logout,
    Features,
    Card,
    CardEu,
    CardEuLight,
    Schedule,
    Notifications,
    NotificationsRead,
    NotificationsRegister,
    NotificationsUnregister,
    Clocking,
    ClockIn,
    Restaurants,
    RestaurantMenus,
    Rss,
    SocialNetwork,
    StaticPages,
    MailCalendar,
    Map,
    ImportantNews,
    ContactUs,
    Contacts,
    AppUpdateInfos,
    Version,
    SsoServiceToken,
    LoginPageContent,
    StatisticsUserAction,
    Health,
}

impl Endpoint {
    pub fn path(&self) -> String {
        match self {
            Self::Login | Self::Logout => "auth".to_string(),
            Self::LoginPageContent => "auth/login-page-content".to_string(),
            Self::SsoServiceToken => "sso-service-token".to_string(),
            Self::StatisticsUserAction => "statistics/user-action".to_string(),
            Self::Features => "features".to_string(),
            Self::Card => "card".to_string(),
            Self::CardEu => "card-eu".to_string(),
            Self::CardEuLight => "card-eu-light".to_string(),
            Self::Schedule => "schedule".to_string(),
            Self::Notifications => "notifications".to_string(),
            Self::NotificationsRead => "notifications/read".to_string(),
            Self::NotificationsRegister => "notifications/register".to_string(),
            Self::NotificationsUnregister => "notifications/unregister".to_string(),
            Self::Clocking => "clocking".to_string(),
            Self::ClockIn => "clock-in".to_string(),
            Self::Restaurants => "restaurants".to_string(),
            Self::RestaurantMenus => "restaurant/menus".to_string(),
            Self::Rss => "rss".to_string(),
            Self::SocialNetwork => "social-network".to_string(),
            Self::StaticPages => "static-pages".to_string(),
            Self::MailCalendar => "mail-calendar".to_string(),
            Self::Map => "map".to_string(),
            Self::ImportantNews => "important-news".to_string(),
            Self::ContactUs => "contact-us".to_string(),
            Self::Contacts => "contacts".to_string(),
            Self::AppUpdateInfos => "app-update-infos".to_string(),
            Self::Version => "version".to_string(),
            Self::Health => "health".to_string(),
        }
    }

    pub fn require_auth(&self) -> bool {
        !matches!(
            self,
            Self::Login
                | Self::LoginPageContent
                | Self::AppUpdateInfos
                | Self::Version
                | Self::Health
                | Self::Rss
                | Self::SocialNetwork
                | Self::StaticPages
                | Self::Map
                | Self::Restaurants
                | Self::RestaurantMenus
        )
    }

    pub fn url(&self, base_url: &Url) -> Result<Url, HttpError> {
        base_url
            .join(self.path().as_str())
            .map_err(HttpError::UrlParsing)
    }
}
