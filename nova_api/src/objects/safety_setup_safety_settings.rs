use crate::objects::limit_settings::LimitSettings;
use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct SafetySetupSafetySettings {
    pub settings: Option<LimitSettings>,
    pub safety_state: Option<String>,
}
