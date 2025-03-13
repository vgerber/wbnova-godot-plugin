use crate::objects::external_joint_stream_datapoint_value::ExternalJointStreamDatapointValue;
use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ExternalJointStreamDatapoint {
    pub value: ExternalJointStreamDatapointValue,
    pub id: i32,
}
