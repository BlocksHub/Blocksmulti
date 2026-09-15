use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, uniffi::Record)]
pub struct ClockingQuery {
    pub username: String,
    pub ip: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, uniffi::Record)]
pub struct ClockingReply {
    pub times: Vec<String>,
    pub day: String,
}
