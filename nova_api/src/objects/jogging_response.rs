use crate::objects::robot_controller_state::RobotControllerState;
use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct JoggingResponse {
    pub motion_group: String,
    pub state: Option<RobotControllerState>,
    pub movement_state: Option<String>,
}
