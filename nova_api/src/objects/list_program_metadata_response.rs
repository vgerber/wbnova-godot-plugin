use crate::objects::program_metadata::ProgramMetadata;
use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ListProgramMetadataResponse {
    pub programs: Vec<ProgramMetadata>,
}
