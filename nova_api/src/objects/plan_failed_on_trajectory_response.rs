use crate::objects::joint_limit_exceeded::JointLimitExceeded;
use crate::objects::joints::Joints;
use crate::objects::out_of_workspace::OutOfWorkspace;
use crate::objects::pose::Pose;
use crate::objects::safety_zone_violation::SafetyZoneViolation;
use crate::objects::singularity::Singularity;
use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct PlanFailedOnTrajectoryResponse {
    pub error_location_on_trajectory: Option<f64>,
    pub safety_zone_violation: Option<SafetyZoneViolation>,
    pub description: Option<String>,
    pub last_valid_joint_position: Option<Joints>,
    pub out_of_workspace: Option<OutOfWorkspace>,
    pub joint_limit_exceeded: Option<JointLimitExceeded>,
    pub singularity: Option<Singularity>,
    pub last_valid_tcp_pose: Option<Pose>,
    pub motion: Option<String>,
}
