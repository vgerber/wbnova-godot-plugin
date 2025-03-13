use crate::objects::limits_override::LimitsOverride;
use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct CommandSettings {
    pub position_blending: Option<f64>,
    pub limits_override: Option<LimitsOverride>,
    pub auto_blending: Option<i32>,
}
