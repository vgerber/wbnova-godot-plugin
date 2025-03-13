use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct MotionGroupInstance {
    pub motion_group: String,
    pub serial_number: Option<String>,
    pub controller: String,
    pub model_from_controller: String,
    pub name_from_controller: String,
}
