use crate::objects::robot_tcp::RobotTcp;
use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ListTcpsResponse {
    pub tcps: Option<Vec<RobotTcp>>,
}
