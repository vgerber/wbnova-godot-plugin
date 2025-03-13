use crate::objects::motion_group_instance::MotionGroupInstance;
use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct MotionGroupInstanceList {
    pub instances: Vec<MotionGroupInstance>,
}
