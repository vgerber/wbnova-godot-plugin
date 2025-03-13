use crate::objects::dh_parameter::DhParameter;
use crate::objects::joint_limit::JointLimit;
use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct MotionGroupSpecification {
    pub dh_parameters: Option<Vec<DhParameter>>,
    pub mechanical_joint_limits: Option<Vec<JointLimit>>,
}
