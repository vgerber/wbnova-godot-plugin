use crate::objects::joints::Joints;
use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct LimitsOverride {
    pub joint_velocity_limits: Option<Joints>,
    pub tcp_orientation_velocity_limit: Option<f64>,
    pub tcp_velocity_limit: Option<f64>,
    pub tcp_acceleration_limit: Option<f64>,
    pub joint_acceleration_limits: Option<Joints>,
    pub tcp_orientation_acceleration_limit: Option<f64>,
}
