use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct EtcdClient {
    pub identifier: Option<String>,
    pub port: Option<i32>,
    #[serde(alias = "type")]
    pub type_name: Option<String>,
    pub host: Option<String>,
}
