use crate::objects::program_run_object::ProgramRunObject;
use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct GetAllProgramRunsOkJson {
    pub program_runs: Option<Vec<ProgramRunObject>>,
}
