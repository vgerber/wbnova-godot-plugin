use crate::objects::get_trajectory_sample_response::GetTrajectorySampleResponse;
use reqwest;

pub enum GetMotionTrajectorySampleResponseType {
    Ok(GetTrajectorySampleResponse),
    UndefinedResponse(reqwest::Response),
}

pub struct GetMotionTrajectorySamplePathParameters {
    pub cell: String,
    pub motion: String,
}
pub struct GetMotionTrajectorySampleQueryParameters {
    pub response_coordinate_system: Option<String>,
    pub location_on_trajectory: Option<f64>,
}

pub async fn get_motion_trajectory_sample(
    client: &reqwest::Client,
    server: &str,
    path_parameters: &GetMotionTrajectorySamplePathParameters,
    query_parameters: &GetMotionTrajectorySampleQueryParameters,
) -> Result<GetMotionTrajectorySampleResponseType, reqwest::Error> {
    let mut request_query_parameters: Vec<(&str, String)> = vec![];
    if let Some(ref query_parameter) = query_parameters.response_coordinate_system {
        request_query_parameters.push(("response_coordinate_system", query_parameter.to_string()));
    }
    if let Some(ref query_parameter) = query_parameters.location_on_trajectory {
        request_query_parameters.push(("location_on_trajectory", query_parameter.to_string()));
    }
    let response = match client
        .get(format!(
            "{server}/cells/{}/motions/{}/trajectorysample",
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
        200 => match response.json::<GetTrajectorySampleResponse>().await {
            Ok(get_trajectory_sample_response) => Ok(GetMotionTrajectorySampleResponseType::Ok(
                get_trajectory_sample_response,
            )),
            Err(parsing_error) => Err(parsing_error),
        },
        _ => Ok(GetMotionTrajectorySampleResponseType::UndefinedResponse(
            response,
        )),
    }
}
