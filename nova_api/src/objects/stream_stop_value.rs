use crate::objects::motion_id::MotionId;
use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct StreamStopValue {
    pub stop: MotionId,
}
