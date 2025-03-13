use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct TriggerObject {
    #[serde(alias = "type")]
    pub type_name: String,
    pub enabled: bool,
    pub last_updated_at: String,
    pub id: Option<String>,
    pub created_at: String,
    pub program_runs: Option<Vec<String>>,
    pub program_id: String,
}
