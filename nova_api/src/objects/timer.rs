use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Timer {
    pub identifier: Option<String>,
    #[serde(alias = "type")]
    pub type_name: Option<String>,
}
