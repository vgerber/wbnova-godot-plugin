use crate::objects::http_validation_error::HttpValidationError;
use reqwest;

pub enum GetDeviceResponseType {
    Ok,
    UnprocessableEntity(HttpValidationError),
    UndefinedResponse(reqwest::Response),
}

pub struct GetDevicePathParameters {
    pub cell: String,
    pub identifier: String,
}

pub async fn get_device(
    client: &reqwest::Client,
    server: &str,
    path_parameters: &GetDevicePathParameters,
) -> Result<GetDeviceResponseType, reqwest::Error> {
    let request_query_parameters: Vec<(&str, String)> = vec![];
    let response = match client
        .get(format!(
            "{server}/cells/{}/devices/{}",
            path_parameters.cell, path_parameters.identifier
        ))
        .query(&request_query_parameters)
        .send()
        .await
    {
        Ok(response) => response,
        Err(err) => return Err(err),
    };
    match response.status().as_u16() {
        200 => Ok(GetDeviceResponseType::Ok),
        422 => match response.json::<HttpValidationError>().await {
            Ok(http_validation_error) => Ok(GetDeviceResponseType::UnprocessableEntity(
                http_validation_error,
            )),
            Err(parsing_error) => Err(parsing_error),
        },
        _ => Ok(GetDeviceResponseType::UndefinedResponse(response)),
    }
}
