use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, uniffi::Record)]
pub struct MailCalendarEvent {
    #[serde(default)]
    pub label: String,
    #[serde(rename = "startDateTime", alias = "start_date_time", default)]
    pub start_date_time: Option<String>,
    #[serde(rename = "endDateTime", alias = "end_date_time", default)]
    pub end_date_time: Option<String>,
    #[serde(default)]
    pub location: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, uniffi::Record)]
pub struct MailCalendarReply {
    #[serde(default)]
    pub error: Option<String>,
    #[serde(rename = "unreadMails", alias = "unread_mails", default)]
    pub unread_mails: u32,
    #[serde(default)]
    pub events: Vec<MailCalendarEvent>,
}
