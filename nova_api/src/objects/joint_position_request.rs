use crate::objects::joints::Joints;
use crate::objects::tcp_pose::TcpPose;
use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct JointPositionRequest {
    pub tcp_pose: TcpPose,
    pub motion_group: String,
    pub reference_joint_position: Joints,
}
