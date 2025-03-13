use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct VirtualRobotConfiguration {
    pub name: String,
    pub content: String,
}
