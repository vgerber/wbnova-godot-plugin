use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct JoggingServiceCapabilities {
    pub joint_jogging: bool,
    pub cartesian_jogging: bool,
}
