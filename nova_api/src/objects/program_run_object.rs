use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ProgramRunObject {
    pub program_id: String,
    pub program_output: Option<String>,
    pub status: String,
    pub created_at: String,
    pub id: String,
    pub last_updated_at: String,
}
