use crate::objects::motion_group_info::MotionGroupInfo;
use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct MotionGroupInfos {
    pub motiongroups: Vec<MotionGroupInfo>,
}
