use crate::objects::programer_runner_reference::ProgramerRunnerReference;
use reqwest;

pub enum ListProgramRunnersResponseType {
    Ok(Vec<ProgramerRunnerReference>),
    UndefinedResponse(reqwest::Response),
}

pub struct ListProgramRunnersPathParameters {
    pub cell: String,
}

pub async fn list_program_runners(
    client: &reqwest::Client,
    server: &str,
    path_parameters: &ListProgramRunnersPathParameters,
) -> Result<ListProgramRunnersResponseType, reqwest::Error> {
    let request_query_parameters: Vec<(&str, String)> = vec![];
    let response = match client
        .get(format!(
            "{server}/cells/{}/programs/runners",
            path_parameters.cell
        ))
        .query(&request_query_parameters)
        .send()
        .await
    {
        Ok(response) => response,
        Err(err) => return Err(err),
    };
    match response.status().as_u16() {
        200 => match response.json::<Vec<ProgramerRunnerReference>>().await {
            Ok(programer_runner_references) => Ok(ListProgramRunnersResponseType::Ok(
                programer_runner_references,
            )),
            Err(parsing_error) => Err(parsing_error),
        },
        _ => Ok(ListProgramRunnersResponseType::UndefinedResponse(response)),
    }
}
