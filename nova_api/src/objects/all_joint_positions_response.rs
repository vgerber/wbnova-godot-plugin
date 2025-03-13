use crate::objects::joints::Joints;
use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct AllJointPositionsResponse {
    pub joint_positions: Option<Vec<Joints>>,
}
