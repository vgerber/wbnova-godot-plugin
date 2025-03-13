use crate::objects::execute_trajectory_request_body_json::ExecuteTrajectoryRequestBodyJson;
use crate::objects::execute_trajectory_response::ExecuteTrajectoryResponse;
use reqwest;

pub enum ExecuteTrajectoryResponseType {
    Ok(ExecuteTrajectoryResponse),
    UndefinedResponse(reqwest::Response),
}

pub struct ExecuteTrajectoryPathParameters {
    pub cell: String,
}

pub async fn execute_trajectory(
    client: &reqwest::Client,
    server: &str,
    content: ExecuteTrajectoryRequestBodyJson,
    path_parameters: &ExecuteTrajectoryPathParameters,
) -> Result<ExecuteTrajectoryResponseType, reqwest::Error> {
    let request_query_parameters: Vec<(&str, String)> = vec![];
    let response = match client
        .get(format!(
            "{server}/cells/{}/motions/executetrajectory",
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
        200 => match response.json::<ExecuteTrajectoryResponse>().await {
            Ok(execute_trajectory_response) => Ok(ExecuteTrajectoryResponseType::Ok(
                execute_trajectory_response,
            )),
            Err(parsing_error) => Err(parsing_error),
        },
        _ => Ok(ExecuteTrajectoryResponseType::UndefinedResponse(response)),
    }
}
