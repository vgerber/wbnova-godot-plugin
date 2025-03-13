use crate::objects::object::Object;
use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Kuka {
    #[serde(alias = "controllerPort")]
    pub controller_port: i32,
    #[serde(alias = "controllerIp")]
    pub controller_ip: String,
    #[serde(alias = "rsiServer")]
    pub rsi_server: Object,
    pub kind: Option<String>,
}
