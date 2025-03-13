use crate::objects::joints::Joints;
use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct TcpPoseRequest {
    pub coordinate_system: Option<String>,
    pub tcp: Option<String>,
    pub motion_group: String,
    pub joint_position: Joints,
}
