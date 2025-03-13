use crate::objects::set_playback_speed::SetPlaybackSpeed;
use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct StreamMovePlaybackSpeed {
    pub playback_speed: SetPlaybackSpeed,
}
