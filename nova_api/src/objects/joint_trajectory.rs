use crate::objects::joints::Joints;
use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct JointTrajectory {
    pub times: Vec<f64>,
    pub locations: Vec<f64>,
    pub joint_positions: Vec<Joints>,
}
