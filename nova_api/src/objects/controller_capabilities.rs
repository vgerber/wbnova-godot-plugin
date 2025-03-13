use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ControllerCapabilities {
    pub support_control: bool,
    pub support_freedrive: bool,
}
