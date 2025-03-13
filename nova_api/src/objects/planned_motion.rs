use crate::objects::joints::Joints;
use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct PlannedMotion {
    pub motion_group: String,
    pub joint_positions: Vec<Joints>,
    pub tcp: Option<String>,
    pub locations: Option<Vec<f64>>,
    pub times: Vec<f64>,
}
