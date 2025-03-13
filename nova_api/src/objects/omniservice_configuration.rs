use crate::objects::prim_paths::PrimPaths;
use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct OmniserviceConfiguration {
    pub prim_paths: Option<PrimPaths>,
    #[serde(alias = "type")]
    pub type_name: Option<String>,
    pub identifier: Option<String>,
    pub host: Option<String>,
}
