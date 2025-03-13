use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ProgramMetadata {
    pub created_date: String,
    pub id: String,
    pub image: Option<String>,
    pub last_updated_date: String,
    pub name: String,
    pub is_hidden: bool,
}
