use crate::objects::validation_error::ValidationError;
use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct HttpValidationError {
    pub detail: Option<Vec<ValidationError>>,
}
