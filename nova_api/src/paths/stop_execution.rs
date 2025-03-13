use reqwest;

pub enum StopExecutionResponseType {
    UndefinedResponse(reqwest::Response),
}

pub struct StopExecutionPathParameters {
    pub motion: String,
    pub cell: String,
}

pub async fn stop_execution(
    client: &reqwest::Client,
    server: &str,
    path_parameters: &StopExecutionPathParameters,
) -> Result<StopExecutionResponseType, reqwest::Error> {
    let request_query_parameters: Vec<(&str, String)> = vec![];
    let response = match client
        .put(format!(
            "{server}/cells/{}/motions/{}/stop",
            path_parameters.motion, path_parameters.cell
        ))
        .query(&request_query_parameters)
        .send()
        .await
    {
        Ok(response) => response,
        Err(err) => return Err(err),
    };
    match response.status().as_u16() {
        _ => Ok(StopExecutionResponseType::UndefinedResponse(response)),
    }
}
