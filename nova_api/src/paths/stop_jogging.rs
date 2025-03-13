use reqwest;

pub enum StopJoggingResponseType {
    UndefinedResponse(reqwest::Response),
}

pub struct StopJoggingPathParameters {
    pub motion_group: String,
    pub cell: String,
}

pub async fn stop_jogging(
    client: &reqwest::Client,
    server: &str,
    path_parameters: &StopJoggingPathParameters,
) -> Result<StopJoggingResponseType, reqwest::Error> {
    let request_query_parameters: Vec<(&str, String)> = vec![];
    let response = match client
        .put(format!(
            "{server}/cells/{}/motion-groups/{}/stop",
            path_parameters.motion_group, path_parameters.cell
        ))
        .query(&request_query_parameters)
        .send()
        .await
    {
        Ok(response) => response,
        Err(err) => return Err(err),
    };
    match response.status().as_u16() {
        _ => Ok(StopJoggingResponseType::UndefinedResponse(response)),
    }
}
