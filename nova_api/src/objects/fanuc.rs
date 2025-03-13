use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Fanuc {
    pub kind: Option<String>,
    #[serde(alias = "controllerIp")]
    pub controller_ip: String,
}
