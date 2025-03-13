use crate::objects::commands_missing::CommandsMissing;
use crate::objects::joint_limit_exceeded::JointLimitExceeded;
use crate::objects::out_of_workspace::OutOfWorkspace;
use crate::objects::safety_zone_violation::SafetyZoneViolation;
use crate::objects::singularity::Singularity;
use crate::objects::start_joints_missing::StartJointsMissing;
use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct PlanFailedResponse {
    pub out_of_workspace: Option<OutOfWorkspace>,
    pub description: Option<String>,
    pub safety_zone_violation: Option<SafetyZoneViolation>,
    pub singularity: Option<Singularity>,
    pub joint_limit_exceeded: Option<JointLimitExceeded>,
    pub commands_missing: Option<CommandsMissing>,
    pub start_joints_missing: Option<StartJointsMissing>,
}
