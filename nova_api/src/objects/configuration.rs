use crate::objects::motion_group_id::MotionGroupId;
use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Configuration {
    pub motion_group_id: Option<MotionGroupId>,
    pub controller_model_name: Option<String>,
    pub rae_host: Option<String>,
    #[serde(alias = "type")]
    pub type_name: Option<String>,
    pub rae_port: Option<i32>,
    pub host: Option<String>,
    pub identifier: Option<String>,
}
