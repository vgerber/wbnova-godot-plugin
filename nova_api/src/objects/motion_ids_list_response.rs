use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct MotionIdsListResponse {
    pub motions: Option<Vec<String>>,
}
