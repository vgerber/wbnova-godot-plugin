use crate::objects::app::App;
use crate::objects::robot_controller::RobotController;
use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Cell {
    pub apps: Option<Vec<App>>,
    pub name: String,
    pub controllers: Option<Vec<RobotController>>,
}
