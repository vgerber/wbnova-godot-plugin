use crate::objects::motion_id::MotionId;
use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct StreamStop {
    pub stop: MotionId,
}
