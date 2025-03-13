use crate::objects::robot_state::RobotState;
use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Path {
    pub poses: Option<Vec<RobotState>>,
}
