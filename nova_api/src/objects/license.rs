use crate::objects::license_status::LicenseStatus;
use crate::objects::object::Object;
use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct License {
    pub owner_email: String,
    pub grace_period_expiry_date: String,
    pub license_expiry_date: Option<String>,
    pub license_key: String,
    pub feature_flags: Option<Vec<String>>,
    pub status: LicenseStatus,
    pub feature_limitations: Option<Object>,
    pub product_name: String,
}
