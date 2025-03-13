use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ModeChangeResponse {
    pub previous_robot_mode: String,
    pub current_robot_mode: String,
    pub by_client_request: Option<bool>,
    pub cause_of_change: String,
}
