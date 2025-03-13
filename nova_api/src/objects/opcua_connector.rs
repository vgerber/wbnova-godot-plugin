use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct OpcuaConnector {
    pub identifier: Option<String>,
    pub url: Option<String>,
    #[serde(alias = "type")]
    pub type_name: Option<String>,
}
