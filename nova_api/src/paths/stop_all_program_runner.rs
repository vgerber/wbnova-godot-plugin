use reqwest;

pub enum StopAllProgramRunnerResponseType {
    Ok,
    UndefinedResponse(reqwest::Response),
}

pub struct StopAllProgramRunnerPathParameters {
    pub cell: String,
}

pub async fn stop_all_program_runner(
    client: &reqwest::Client,
    server: &str,
    path_parameters: &StopAllProgramRunnerPathParameters,
) -> Result<StopAllProgramRunnerResponseType, reqwest::Error> {
    let request_query_parameters: Vec<(&str, String)> = vec![];
    let response = match client
        .post(format!(
            "{server}/cells/{}/programs/runners/stop",
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
        200 => Ok(StopAllProgramRunnerResponseType::Ok),
        _ => Ok(StopAllProgramRunnerResponseType::UndefinedResponse(
            response,
        )),
    }
}
