use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct CreateTriggerRequestBodyJson {
    #[serde(alias = "type")]
    pub type_name: String,
    pub program_id: String,
    pub enabled: bool,
}
