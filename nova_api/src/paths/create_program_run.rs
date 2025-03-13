use crate::objects::create_program_run_ok_json::CreateProgramRunOkJson;
use crate::objects::create_program_run_request_body_json::CreateProgramRunRequestBodyJson;
use reqwest;

pub enum CreateProgramRunResponseType {
    Ok(CreateProgramRunOkJson),
    UndefinedResponse(reqwest::Response),
}

pub struct CreateProgramRunPathParameters {
    pub cell: String,
}

pub async fn create_program_run(
    client: &reqwest::Client,
    server: &str,
    content: CreateProgramRunRequestBodyJson,
    path_parameters: &CreateProgramRunPathParameters,
) -> Result<CreateProgramRunResponseType, reqwest::Error> {
    let request_query_parameters: Vec<(&str, String)> = vec![];
    let response = match client
        .post(format!(
            "{server}/cells/{}/operator/programs/runs",
            path_parameters.cell
        ))
        .query(&request_query_parameters)
        .json(&content)
        .send()
        .await
    {
        Ok(response) => response,
        Err(err) => return Err(err),
    };
    match response.status().as_u16() {
        200 => match response.json::<CreateProgramRunOkJson>().await {
            Ok(create_program_run_ok_json) => {
                Ok(CreateProgramRunResponseType::Ok(create_program_run_ok_json))
            }
            Err(parsing_error) => Err(parsing_error),
        },
        _ => Ok(CreateProgramRunResponseType::UndefinedResponse(response)),
    }
}
