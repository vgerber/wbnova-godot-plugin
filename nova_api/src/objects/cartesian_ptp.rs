use crate::objects::pose::Pose;
use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct CartesianPtp {
    pub target_pose: Pose,
    pub path_definition_name: String,
}
