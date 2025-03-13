use crate::objects::program_run_object::ProgramRunObject;
use reqwest;

pub enum GetProgramRunResponseType {
    Ok(ProgramRunObject),
    UndefinedResponse(reqwest::Response),
}

pub struct GetProgramRunPathParameters {
    pub run: String,
    pub cell: String,
}

pub async fn get_program_run(
    client: &reqwest::Client,
    server: &str,
    path_parameters: &GetProgramRunPathParameters,
) -> Result<GetProgramRunResponseType, reqwest::Error> {
    let request_query_parameters: Vec<(&str, String)> = vec![];
    let response = match client
        .get(format!(
            "{server}/cells/{}/operator/programs/runs/{}",
            path_parameters.run, path_parameters.cell
        ))
        .query(&request_query_parameters)
        .send()
        .await
    {
        Ok(response) => response,
        Err(err) => return Err(err),
    };
    match response.status().as_u16() {
        200 => match response.json::<ProgramRunObject>().await {
            Ok(program_run_object) => Ok(GetProgramRunResponseType::Ok(program_run_object)),
            Err(parsing_error) => Err(parsing_error),
        },
        _ => Ok(GetProgramRunResponseType::UndefinedResponse(response)),
    }
}
