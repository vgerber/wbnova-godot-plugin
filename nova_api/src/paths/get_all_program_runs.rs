use crate::objects::get_all_program_runs_ok_json::GetAllProgramRunsOkJson;
use reqwest;

pub enum GetAllProgramRunsResponseType {
    Ok(GetAllProgramRunsOkJson),
    UndefinedResponse(reqwest::Response),
}

pub struct GetAllProgramRunsPathParameters {
    pub cell: String,
}
pub struct GetAllProgramRunsQueryParameters {
    pub state: Option<String>,
}

pub async fn get_all_program_runs(
    client: &reqwest::Client,
    server: &str,
    path_parameters: &GetAllProgramRunsPathParameters,
    query_parameters: &GetAllProgramRunsQueryParameters,
) -> Result<GetAllProgramRunsResponseType, reqwest::Error> {
    let mut request_query_parameters: Vec<(&str, String)> = vec![];
    if let Some(ref query_parameter) = query_parameters.state {
        request_query_parameters.push(("state", query_parameter.to_string()));
    }
    let response = match client
        .get(format!(
            "{server}/cells/{}/operator/programs/runs",
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
        200 => match response.json::<GetAllProgramRunsOkJson>().await {
            Ok(get_all_program_runs_ok_json) => Ok(GetAllProgramRunsResponseType::Ok(
                get_all_program_runs_ok_json,
            )),
            Err(parsing_error) => Err(parsing_error),
        },
        _ => Ok(GetAllProgramRunsResponseType::UndefinedResponse(response)),
    }
}
