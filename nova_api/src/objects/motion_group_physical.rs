use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct MotionGroupPhysical {
    pub name_from_controller: String,
    pub motion_group: String,
    pub active: bool,
    pub model_from_controller: Option<String>,
    pub serial_number: Option<String>,
}
