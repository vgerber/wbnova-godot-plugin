use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct SetPlaybackSpeed {
    pub motion: String,
    pub playback_speed_in_percent: i32,
}
