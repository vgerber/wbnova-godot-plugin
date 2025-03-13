use crate::objects::object::Object;
use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Abb {
    #[serde(alias = "egmServer")]
    pub egm_server: Object,
    #[serde(alias = "controllerPort")]
    pub controller_port: i32,
    pub kind: Option<String>,
    #[serde(alias = "controllerIp")]
    pub controller_ip: String,
}
