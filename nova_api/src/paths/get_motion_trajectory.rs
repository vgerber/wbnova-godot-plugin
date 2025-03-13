use crate::objects::get_trajectory_response::GetTrajectoryResponse;
use reqwest;

pub enum GetMotionTrajectoryResponseType {
    Ok(GetTrajectoryResponse),
    UndefinedResponse(reqwest::Response),
}

pub struct GetMotionTrajectoryPathParameters {
    pub cell: String,
    pub motion: String,
}
pub struct GetMotionTrajectoryQueryParameters {
    pub sample_time: i32,
    pub responses_coordinate_system: Option<String>,
}

pub async fn get_motion_trajectory(
    client: &reqwest::Client,
    server: &str,
    path_parameters: &GetMotionTrajectoryPathParameters,
    query_parameters: &GetMotionTrajectoryQueryParameters,
) -> Result<GetMotionTrajectoryResponseType, reqwest::Error> {
    let mut request_query_parameters: Vec<(&str, String)> =
        vec![("sample_time", query_parameters.sample_time.to_string())];
    if let Some(ref query_parameter) = query_parameters.responses_coordinate_system {
        request_query_parameters.push(("responses_coordinate_system", query_parameter.to_string()));
    }
    let response = match client
        .get(format!(
            "{server}/cells/{}/motions/{}/trajectory",
            path_parameters.cell, path_parameters.motion
        ))
        .query(&request_query_parameters)
        .send()
        .await
    {
        Ok(response) => response,
        Err(err) => return Err(err),
    };
    match response.status().as_u16() {
        200 => match response.json::<GetTrajectoryResponse>().await {
            Ok(get_trajectory_response) => {
                Ok(GetMotionTrajectoryResponseType::Ok(get_trajectory_response))
            }
            Err(parsing_error) => Err(parsing_error),
        },
        _ => Ok(GetMotionTrajectoryResponseType::UndefinedResponse(response)),
    }
}
