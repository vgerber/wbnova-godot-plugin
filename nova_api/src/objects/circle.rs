use crate::objects::pose::Pose;
use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Circle {
    pub target_pose: Pose,
    pub via_pose: Pose,
}
