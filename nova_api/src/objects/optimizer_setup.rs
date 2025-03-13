use crate::objects::dh_parameter::DhParameter;
use crate::objects::payload::Payload;
use crate::objects::planner_pose::PlannerPose;
use crate::objects::safety_configuration::SafetyConfiguration;
use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct OptimizerSetup {
    pub cycle_time: Option<i32>,
    pub motion_group_type: String,
    pub payload: Option<Payload>,
    pub safety_setup: SafetyConfiguration,
    pub tcp: PlannerPose,
    pub mounting: PlannerPose,
    pub dh_parameters: Option<Vec<DhParameter>>,
}
