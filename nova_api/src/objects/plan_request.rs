use crate::objects::command::Command;
use crate::objects::joints::Joints;
use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct PlanRequest {
    pub commands: Vec<Command>,
    pub start_joint_position: Joints,
    pub tcp: Option<String>,
    pub motion_group: String,
    pub payload: Option<String>,
}
