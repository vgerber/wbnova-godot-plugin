use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct PauseMovementRequest {
    pub send_response: Option<bool>,
    pub message_type: Option<String>,
}
