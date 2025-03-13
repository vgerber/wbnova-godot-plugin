use crate::objects::joints::Joints;
use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct PlanSuccessfulResponse {
    pub end_joint_position: Joints,
    pub motion: String,
}
