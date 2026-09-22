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
    pub fn path(&self) -> &'static str {
        match self {
            Self::Login | Self::Logout => "auth",
            Self::LoginPageContent => "auth/login-page-content",
            Self::SsoServiceToken => "sso-service-token",
            Self::StatisticsUserAction => "statistics/user-action",
            Self::Features => "features",
            Self::Card => "card",
            Self::CardEu => "card-eu",
            Self::CardEuLight => "card-eu-light",
            Self::Schedule => "schedule",
            Self::Notifications => "notifications",
            Self::NotificationsRead => "notifications/read",
            Self::NotificationsRegister => "notifications/register",
            Self::NotificationsUnregister => "notifications/unregister",
            Self::Clocking => "clocking",
            Self::ClockIn => "clock-in",
            Self::Restaurants => "restaurants",
            Self::RestaurantMenus => "restaurant/menus",
            Self::Rss => "rss",
            Self::SocialNetwork => "social-network",
            Self::StaticPages => "static-pages",
            Self::MailCalendar => "mail-calendar",
            Self::Map => "map",
            Self::ImportantNews => "important-news",
            Self::ContactUs => "contact-us",
            Self::Contacts => "contacts",
            Self::AppUpdateInfos => "app-update-infos",
            Self::Version => "version",
            Self::Health => "health",
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
            .join(self.path())
            .map_err(HttpError::UrlParsing)
    }
}
