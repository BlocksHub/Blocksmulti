use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, uniffi::Record)]
pub struct ScheduleMessage {
    pub level: String,
    pub code: Option<String>,
    pub text: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, uniffi::Record)]
pub struct PlanningMessage {
    pub level: String,
    pub text: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, uniffi::Record)]
pub struct Course {
    pub id: String,
    pub label: String,
    pub color: String,
    #[serde(rename = "type")]
    pub course_type: String,
    pub online: bool,
    pub url: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, uniffi::Record)]
pub struct Room {
    pub id: String,
    pub label: String,
    #[serde(rename = "type")]
    pub room_type: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, uniffi::Record)]
pub struct Teacher {
    pub id: String,
    pub firstname: String,
    pub lastname: String,
    pub email: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, uniffi::Record)]
pub struct Group {
    pub id: String,
    pub label: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, uniffi::Record)]
pub struct ScheduleEvent {
    pub id: String,
    #[serde(rename = "startDateTime")]
    pub start_date_time: String,
    #[serde(rename = "endDateTime")]
    pub end_date_time: String,
    #[serde(rename = "planningLabel")]
    pub planning_label: String,
    pub course: Course,
    pub rooms: Vec<Room>,
    pub teachers: Vec<Teacher>,
    pub groups: Vec<Group>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, uniffi::Record)]
pub struct Planning {
    pub id: String,
    pub label: String,
    pub default: bool,
    #[serde(rename = "type")]
    pub planning_type: String,
    pub messages: Vec<PlanningMessage>,
    pub events: Option<Vec<ScheduleEvent>>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, uniffi::Record)]
pub struct Schedule {
    pub messages: Vec<ScheduleMessage>,
    pub plannings: Option<Vec<Planning>>,
}
