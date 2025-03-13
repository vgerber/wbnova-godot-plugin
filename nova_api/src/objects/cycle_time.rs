use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct CycleTime {
    pub cycle_time_ms: i32,
}
