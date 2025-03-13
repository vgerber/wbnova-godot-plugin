use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct PlanningLimitsLimitRange {
    pub lower_limit: f64,
    pub upper_limit: f64,
}
