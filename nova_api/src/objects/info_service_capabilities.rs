use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct InfoServiceCapabilities {
    pub get_active_tcp: bool,
    pub list_tcps: bool,
    pub get_active_payload: bool,
    pub get_mounting: bool,
    pub get_safety_setup: bool,
    pub get_motion_group_specification: bool,
    pub list_payloads: bool,
}
