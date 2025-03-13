use crate::objects::move_response::MoveResponse;
use crate::objects::robot_controller_state::RobotControllerState;
use crate::objects::stop_response::StopResponse;
use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct StreamMoveResponse {
    pub state: Option<RobotControllerState>,
    pub move_response: Option<MoveResponse>,
    pub stop_response: Option<StopResponse>,
}
