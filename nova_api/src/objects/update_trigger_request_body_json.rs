use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct UpdateTriggerRequestBodyJson {
    pub enabled: Option<bool>,
    pub program_id: Option<String>,
}
