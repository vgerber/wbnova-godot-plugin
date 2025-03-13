use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct SafetyZoneViolation {
    pub description: Option<String>,
}
