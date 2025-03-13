use crate::objects::plan_failed_on_trajectory_response::PlanFailedOnTrajectoryResponse;
use crate::objects::plan_failed_response::PlanFailedResponse;
use crate::objects::plan_successful_response::PlanSuccessfulResponse;
use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct PlanResponse {
    pub plan_failed_response: Option<PlanFailedResponse>,
    pub plan_failed_on_trajectory_response: Option<PlanFailedOnTrajectoryResponse>,
    pub plan_successful_response: Option<PlanSuccessfulResponse>,
}
