use crate::objects::joints::Joints;
use crate::objects::pose::Pose;
use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct TrajectorySample {
    pub time: Option<f64>,
    pub joint_velocity: Option<Joints>,
    pub joint_acceleration: Option<Joints>,
    pub joint_position: Option<Joints>,
    pub tcp_velocity: Option<f64>,
    pub joint_torques: Option<Joints>,
    pub tcp_acceleration: Option<f64>,
    pub tcp_orientation_acceleration: Option<f64>,
    pub tcp_orientation_velocity: Option<f64>,
    pub tcp_pose: Option<Pose>,
    pub location_on_trajectory: Option<f64>,
}
