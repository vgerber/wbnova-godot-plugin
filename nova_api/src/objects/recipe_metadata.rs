use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct RecipeMetadata {
    pub last_updated_date: String,
    pub is_production: bool,
    pub program_id: String,
    pub name: String,
    pub image: Option<String>,
    pub created_date: String,
    pub id: Option<String>,
}
