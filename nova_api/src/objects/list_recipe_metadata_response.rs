use crate::objects::recipe_metadata::RecipeMetadata;
use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ListRecipeMetadataResponse {
    pub recipes: Vec<RecipeMetadata>,
}
