use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct KinematicServiceCapabilities {
    pub calculate_joint_position: bool,
    pub calculate_all_joint_positions: bool,
    pub calculate_tcp_pose: bool,
}
