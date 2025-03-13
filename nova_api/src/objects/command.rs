use crate::objects::circle::Circle;
use crate::objects::command_settings::CommandSettings;
use crate::objects::cubic_spline::CubicSpline;
use crate::objects::joints::Joints;
use crate::objects::pose::Pose;
use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Command {
    pub cubic_spline: Option<CubicSpline>,
    pub circle: Option<Circle>,
    pub joint_ptp: Option<Joints>,
    pub line: Option<Pose>,
    pub cartesian_ptp: Option<Pose>,
    pub settings: Option<CommandSettings>,
}
