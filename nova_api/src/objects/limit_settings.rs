use crate::objects::joint_limit::JointLimit;
use crate::objects::single_joint_limit::SingleJointLimit;
use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct LimitSettings {
    pub elbow_force_limit: Option<f64>,
    pub elbow_acceleration_limit: Option<f64>,
    pub joint_position_limits: Option<Vec<JointLimit>>,
    pub tcp_velocity_limit: Option<f64>,
    pub elbow_velocity_limit: Option<f64>,
    pub tcp_orientation_acceleration_limit: Option<f64>,
    pub tcp_acceleration_limit: Option<f64>,
    pub tcp_orientation_velocity_limit: Option<f64>,
    pub joint_velocity_limits: Option<Vec<SingleJointLimit>>,
    pub joint_torque_limits: Option<Vec<SingleJointLimit>>,
    pub tcp_force_limit: Option<f64>,
    pub joint_acceleration_limits: Option<Vec<SingleJointLimit>>,
}
