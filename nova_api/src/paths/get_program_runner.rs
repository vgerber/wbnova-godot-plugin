use crate::objects::http_validation_error::HttpValidationError;
use crate::objects::program_run::ProgramRun;
use reqwest;

pub enum GetProgramRunnerResponseType {
    UnprocessableEntity(HttpValidationError),
    Ok(ProgramRun),
    UndefinedResponse(reqwest::Response),
}

pub struct GetProgramRunnerPathParameters {
    pub runner: String,
    pub cell: String,
}

pub async fn get_program_runner(
    client: &reqwest::Client,
    server: &str,
    path_parameters: &GetProgramRunnerPathParameters,
) -> Result<GetProgramRunnerResponseType, reqwest::Error> {
    let request_query_parameters: Vec<(&str, String)> = vec![];
    let response = match client
        .get(format!(
            "{server}/cells/{}/programs/runners/{}",
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
            Ok(http_validation_error) => Ok(GetProgramRunnerResponseType::UnprocessableEntity(
                http_validation_error,
            )),
            Err(parsing_error) => Err(parsing_error),
        },
        200 => match response.json::<ProgramRun>().await {
            Ok(program_run) => Ok(GetProgramRunnerResponseType::Ok(program_run)),
            Err(parsing_error) => Err(parsing_error),
        },
        _ => Ok(GetProgramRunnerResponseType::UndefinedResponse(response)),
    }
}
