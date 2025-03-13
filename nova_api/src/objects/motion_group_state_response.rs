use crate::objects::motion_group_state::MotionGroupState;
use crate::objects::tcp_pose::TcpPose;
use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct MotionGroupStateResponse {
    pub state: MotionGroupState,
    pub tcp_pose: Option<TcpPose>,
}
