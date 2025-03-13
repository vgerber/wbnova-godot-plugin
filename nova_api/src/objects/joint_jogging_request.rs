use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct JointJoggingRequest {
    pub motion_group: String,
    pub joint_velocities: Vec<f64>,
    pub response_rate: Option<i32>,
    pub response_coordinate_system: Option<String>,
}
