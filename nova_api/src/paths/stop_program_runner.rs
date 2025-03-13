use crate::objects::http_validation_error::HttpValidationError;
use reqwest;

pub enum StopProgramRunnerResponseType {
    UnprocessableEntity(HttpValidationError),
    UndefinedResponse(reqwest::Response),
}

pub struct StopProgramRunnerPathParameters {
    pub runner: String,
    pub cell: String,
}

pub async fn stop_program_runner(
    client: &reqwest::Client,
    server: &str,
    path_parameters: &StopProgramRunnerPathParameters,
) -> Result<StopProgramRunnerResponseType, reqwest::Error> {
    let request_query_parameters: Vec<(&str, String)> = vec![];
    let response = match client
        .post(format!(
            "{server}/cells/{}/programs/runners/{}/stop",
            path_parameters.runner, path_parameters.cell
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
            Ok(http_validation_error) => Ok(StopProgramRunnerResponseType::UnprocessableEntity(
                http_validation_error,
            )),
            Err(parsing_error) => Err(parsing_error),
        },
        _ => Ok(StopProgramRunnerResponseType::UndefinedResponse(response)),
    }
}
