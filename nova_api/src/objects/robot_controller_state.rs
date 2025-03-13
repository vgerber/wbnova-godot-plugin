use crate::objects::motion_group_state::MotionGroupState;
use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct RobotControllerState {
    pub operation_mode: String,
    pub motion_groups: Vec<MotionGroupState>,
    pub controller: String,
    pub timestamp: String,
    pub velocity_override: Option<i32>,
    pub safety_state: String,
}
