use crate::objects::force_vector::ForceVector;
use crate::objects::joints::Joints;
use crate::objects::motion_group_state_joint_limit_reached::MotionGroupStateJointLimitReached;
use crate::objects::motion_vector::MotionVector;
use crate::objects::pose::Pose;
use crate::objects::tcp_pose::TcpPose;
use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct MotionGroupState {
    pub force: Option<ForceVector>,
    pub tcp_pose: TcpPose,
    pub flange_pose: Option<Pose>,
    pub velocity: MotionVector,
    pub joint_current: Option<Joints>,
    pub joint_limit_reached: MotionGroupStateJointLimitReached,
    pub joint_velocity: Joints,
    pub motion_group: String,
    pub controller: String,
    pub joint_position: Joints,
    pub joint_torque: Option<Joints>,
}
