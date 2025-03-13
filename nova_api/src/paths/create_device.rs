use crate::objects::http_validation_error::HttpValidationError;
use reqwest;

pub enum CreateDeviceResponseType {
    UnprocessableEntity(HttpValidationError),
    Ok,
    UndefinedResponse(reqwest::Response),
}

pub struct CreateDevicePathParameters {
    pub cell: String,
}

pub async fn create_device(
    client: &reqwest::Client,
    server: &str,
    path_parameters: &CreateDevicePathParameters,
) -> Result<CreateDeviceResponseType, reqwest::Error> {
    let request_query_parameters: Vec<(&str, String)> = vec![];
    let response = match client
        .post(format!("{server}/cells/{}/devices", path_parameters.cell))
        .query(&request_query_parameters)
        .send()
        .await
    {
        Ok(response) => response,
        Err(err) => return Err(err),
    };
    match response.status().as_u16() {
        422 => match response.json::<HttpValidationError>().await {
            Ok(http_validation_error) => Ok(CreateDeviceResponseType::UnprocessableEntity(
                http_validation_error,
            )),
            Err(parsing_error) => Err(parsing_error),
        },
        200 => Ok(CreateDeviceResponseType::Ok),
        _ => Ok(CreateDeviceResponseType::UndefinedResponse(response)),
    }
}
