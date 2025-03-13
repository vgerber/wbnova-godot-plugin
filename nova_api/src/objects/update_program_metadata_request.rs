use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct UpdateProgramMetadataRequest {
    pub is_hidden: Option<bool>,
    pub image: Option<String>,
    pub name: Option<String>,
}
