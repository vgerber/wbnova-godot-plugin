use crate::objects::motion_group_physical::MotionGroupPhysical;
use crate::objects::version_number::VersionNumber;
use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ControllerInstance {
    pub model_name: String,
    pub host: String,
    pub error_details: Option<String>,
    pub physical_motion_groups: Vec<MotionGroupPhysical>,
    pub has_error: bool,
    pub vendor_software_version: Option<VersionNumber>,
    pub allow_software_install_on_controller: bool,
    pub controller: String,
}
