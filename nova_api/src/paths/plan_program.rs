use crate::objects::http_validation_error::HttpValidationError;
use crate::objects::program_run::ProgramRun;
use reqwest;

pub enum PlanProgramResponseType {
    UnprocessableEntity(HttpValidationError),
    Ok(ProgramRun),
    UndefinedResponse(reqwest::Response),
}

pub struct PlanProgramPathParameters {
    pub cell: String,
}
pub struct PlanProgramQueryParameters {
    pub identifier: Option<String>,
}

pub async fn plan_program(
    client: &reqwest::Client,
    server: &str,
    content: &String,
    path_parameters: &PlanProgramPathParameters,
    query_parameters: &PlanProgramQueryParameters,
) -> Result<PlanProgramResponseType, reqwest::Error> {
    let mut request_query_parameters: Vec<(&str, String)> = vec![];
    if let Some(ref query_parameter) = query_parameters.identifier {
        request_query_parameters.push(("identifier", query_parameter.to_string()));
    }
    let body = content.to_owned();
    let response = match client
        .post(format!(
            "{server}/cells/{}/programs/plan",
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
        422 => match response.json::<HttpValidationError>().await {
            Ok(http_validation_error) => Ok(PlanProgramResponseType::UnprocessableEntity(
                http_validation_error,
            )),
            Err(parsing_error) => Err(parsing_error),
        },
        200 => match response.json::<ProgramRun>().await {
            Ok(program_run) => Ok(PlanProgramResponseType::Ok(program_run)),
            Err(parsing_error) => Err(parsing_error),
        },
        _ => Ok(PlanProgramResponseType::UndefinedResponse(response)),
    }
}
