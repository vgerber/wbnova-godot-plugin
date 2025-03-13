use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct OpcuaNodeValueTriggerConfig {
    pub node_id: String,
    pub host: String,
}
