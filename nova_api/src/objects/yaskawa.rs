use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Yaskawa {
    #[serde(alias = "controllerIp")]
    pub controller_ip: String,
    pub kind: Option<String>,
}
