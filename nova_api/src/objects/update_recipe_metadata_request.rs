use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct UpdateRecipeMetadataRequest {
    pub program_id: Option<String>,
    pub name: Option<String>,
    pub is_production: Option<bool>,
}
