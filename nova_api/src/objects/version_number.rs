use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct VersionNumber {
    pub string_version: Option<String>,
    pub build_version: Option<i32>,
    pub build_version_wildcard: Option<bool>,
    pub major_version: i32,
    pub minor_version_wildcard: Option<bool>,
    pub bugfix_version_wildcard: Option<bool>,
    pub bugfix_version: Option<i32>,
    pub minor_version: Option<i32>,
}
