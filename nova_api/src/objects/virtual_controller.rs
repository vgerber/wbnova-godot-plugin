use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct VirtualController {
    pub manufacturer: String,
    pub position: Option<String>,
    pub kind: Option<String>,
    #[serde(alias = "type")]
    pub type_name: Option<String>,
    pub json: Option<String>,
}
