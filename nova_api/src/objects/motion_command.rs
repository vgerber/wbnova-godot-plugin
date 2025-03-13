use crate::objects::limits_override::LimitsOverride;
use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct MotionCommand {
    pub limits_override: Option<LimitsOverride>,
}
