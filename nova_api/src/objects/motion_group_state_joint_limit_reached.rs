use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct MotionGroupStateJointLimitReached {
    pub limit_reached: Vec<bool>,
}
