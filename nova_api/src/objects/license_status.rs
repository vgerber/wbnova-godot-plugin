use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct LicenseStatus {
    pub message: String,
    pub status: String,
}
