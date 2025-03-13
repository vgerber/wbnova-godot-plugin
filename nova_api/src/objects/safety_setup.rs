use crate::objects::robot_link_geometry::RobotLinkGeometry;
use crate::objects::safety_setup_safety_settings::SafetySetupSafetySettings;
use crate::objects::safety_setup_safety_zone::SafetySetupSafetyZone;
use crate::objects::tool_geometry::ToolGeometry;
use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct SafetySetup {
    pub robot_model_geometries: Option<Vec<RobotLinkGeometry>>,
    pub tool_geometries: Option<Vec<ToolGeometry>>,
    pub safety_zones: Option<Vec<SafetySetupSafetyZone>>,
    pub safety_settings: Option<Vec<SafetySetupSafetySettings>>,
}
