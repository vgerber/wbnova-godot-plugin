use crate::objects::move_to_trajectory_via_joint_ptp_request::MoveToTrajectoryViaJointPtpRequest;
use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct StreamMoveToTrajectory {
    pub to_trajectory: MoveToTrajectoryViaJointPtpRequest,
}
