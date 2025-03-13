use crate::objects::limits_override::LimitsOverride;
use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct MoveToTrajectoryViaJointPtpRequest {
    pub limit_override: Option<LimitsOverride>,
    pub response_coordinate_system: Option<String>,
    pub location_on_trajectory: f64,
    pub motion: String,
}
