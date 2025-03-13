use crate::objects::http_validation_error::HttpValidationError;
use reqwest;

pub enum DeleteDeviceResponseType {
    UnprocessableEntity(HttpValidationError),
    UndefinedResponse(reqwest::Response),
}

pub struct DeleteDevicePathParameters {
    pub identifier: String,
    pub cell: String,
}

pub async fn delete_device(
    client: &reqwest::Client,
    server: &str,
    path_parameters: &DeleteDevicePathParameters,
) -> Result<DeleteDeviceResponseType, reqwest::Error> {
    let request_query_parameters: Vec<(&str, String)> = vec![];
    let response = match client
        .delete(format!(
            "{server}/cells/{}/devices/{}",
            path_parameters.identifier, path_parameters.cell
        ))
        .query(&request_query_parameters)
        .send()
        .await
    {
        Ok(response) => response,
        Err(err) => return Err(err),
    };
    match response.status().as_u16() {
        422 => match response.json::<HttpValidationError>().await {
            Ok(http_validation_error) => Ok(DeleteDeviceResponseType::UnprocessableEntity(
                http_validation_error,
            )),
            Err(parsing_error) => Err(parsing_error),
        },
        _ => Ok(DeleteDeviceResponseType::UndefinedResponse(response)),
    }
}
