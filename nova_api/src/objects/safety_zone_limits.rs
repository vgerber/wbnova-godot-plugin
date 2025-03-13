use crate::objects::planning_limits::PlanningLimits;
use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct SafetyZoneLimits {
    pub safety_zone: i32,
    pub limits: PlanningLimits,
}
