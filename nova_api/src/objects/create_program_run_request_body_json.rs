use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct CreateProgramRunRequestBodyJson {
    pub program_id: String,
}
