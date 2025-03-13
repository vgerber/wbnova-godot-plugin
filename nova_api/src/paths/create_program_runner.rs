use crate::objects::http_validation_error::HttpValidationError;
use crate::objects::programer_runner_reference::ProgramerRunnerReference;
use reqwest;

pub enum CreateProgramRunnerResponseType {
    Ok(ProgramerRunnerReference),
    UnprocessableEntity(HttpValidationError),
    UndefinedResponse(reqwest::Response),
}

pub struct CreateProgramRunnerPathParameters {
    pub cell: String,
}

pub async fn create_program_runner(
    client: &reqwest::Client,
    server: &str,
    content: &String,
    path_parameters: &CreateProgramRunnerPathParameters,
) -> Result<CreateProgramRunnerResponseType, reqwest::Error> {
    let request_query_parameters: Vec<(&str, String)> = vec![];
    let body = content.to_owned();
    let response = match client
        .post(format!(
            "{server}/cells/{}/programs/runners",
            path_parameters.cell
        ))
        .query(&request_query_parameters)
        .body(body)
        .send()
        .await
    {
        Ok(response) => response,
        Err(err) => return Err(err),
    };
    match response.status().as_u16() {
        200 => match response.json::<ProgramerRunnerReference>().await {
            Ok(programer_runner_reference) => Ok(CreateProgramRunnerResponseType::Ok(
                programer_runner_reference,
            )),
            Err(parsing_error) => Err(parsing_error),
        },
        422 => match response.json::<HttpValidationError>().await {
            Ok(http_validation_error) => Ok(CreateProgramRunnerResponseType::UnprocessableEntity(
                http_validation_error,
            )),
            Err(parsing_error) => Err(parsing_error),
        },
        _ => Ok(CreateProgramRunnerResponseType::UndefinedResponse(response)),
    }
}
