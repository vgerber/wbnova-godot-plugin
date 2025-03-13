use crate::objects::trajectory_sample::TrajectorySample;
use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct GetTrajectoryResponse {
    pub trajectory: Option<Vec<TrajectorySample>>,
}
