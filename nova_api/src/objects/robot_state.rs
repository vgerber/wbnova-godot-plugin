use crate::objects::joints::Joints;
use crate::objects::pose::Pose;
use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct RobotState {
    pub pose: Pose,
    pub joints: Option<Joints>,
}
