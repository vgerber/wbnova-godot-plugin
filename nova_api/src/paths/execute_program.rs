use crate::objects::code_with_arguments::CodeWithArguments;
use crate::objects::http_validation_error::HttpValidationError;
use crate::objects::program_run::ProgramRun;
use reqwest;

pub enum ExecuteProgramResponseType {
    UnprocessableEntity(HttpValidationError),
    Ok(ProgramRun),
    UndefinedResponse(reqwest::Response),
}

pub struct ExecuteProgramPathParameters {
    pub cell: String,
}

pub async fn execute_program_text(
    client: &reqwest::Client,
    server: &str,
    path_parameters: &ExecuteProgramPathParameters,
    content: &String,
) -> Result<ExecuteProgramResponseType, reqwest::Error> {
    let body = content.to_owned();
    let request_builder = client
        .post(format!(
            "{server}/cells/{}/programs/execute",
            path_parameters.cell
        ))
        .body(body);
    execute_program(request_builder).await
}
pub async fn execute_program_json(
    client: &reqwest::Client,
    server: &str,
    path_parameters: &ExecuteProgramPathParameters,
    content: CodeWithArguments,
) -> Result<ExecuteProgramResponseType, reqwest::Error> {
    let request_builder = client
        .post(format!(
            "{server}/cells/{}/programs/execute",
            path_parameters.cell
        ))
        .json(&content);
    execute_program(request_builder).await
}

async fn execute_program(
    request_builder: reqwest::RequestBuilder,
) -> Result<ExecuteProgramResponseType, reqwest::Error> {
    let request_query_parameters: Vec<(&str, String)> = vec![];
    let response = match request_builder
        .query(&request_query_parameters)
        .send()
        .await
    {
        Ok(response) => response,
        Err(err) => return Err(err),
    };
    match response.status().as_u16() {
        422 => match response.json::<HttpValidationError>().await {
            Ok(http_validation_error) => Ok(ExecuteProgramResponseType::UnprocessableEntity(
                http_validation_error,
            )),
            Err(parsing_error) => Err(parsing_error),
        },
        200 => match response.json::<ProgramRun>().await {
            Ok(program_run) => Ok(ExecuteProgramResponseType::Ok(program_run)),
            Err(parsing_error) => Err(parsing_error),
        },
        _ => Ok(ExecuteProgramResponseType::UndefinedResponse(response)),
    }
}
