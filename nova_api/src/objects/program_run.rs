use crate::objects::end_time::EndTime;
use crate::objects::error::Error;
use crate::objects::execution_result::ExecutionResult;
use crate::objects::start_time::StartTime;
use crate::objects::store::Store;
use crate::objects::traceback::Traceback;
use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ProgramRun {
    pub error: Option<Error>,
    pub logs: Option<String>,
    pub state: String,
    pub id: String,
    pub traceback: Option<Traceback>,
    pub start_time: Option<StartTime>,
    pub end_time: Option<EndTime>,
    pub stdout: Option<String>,
    pub store: Option<Store>,
    pub execution_results: Option<Vec<ExecutionResult>>,
}
