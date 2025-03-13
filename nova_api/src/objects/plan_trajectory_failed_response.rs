use crate::objects::joint_trajectory::JointTrajectory;
use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct PlanTrajectoryFailedResponse {
    pub error_location_on_trajectory: Option<f64>,
    pub joint_trajectory: Option<JointTrajectory>,
}
