use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct BodyUploadProgramMetadataImage {
    pub file: String,
}
