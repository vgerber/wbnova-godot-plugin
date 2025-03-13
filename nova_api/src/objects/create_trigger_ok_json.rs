use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct CreateTriggerOkJson {
    pub id: Option<String>,
}
