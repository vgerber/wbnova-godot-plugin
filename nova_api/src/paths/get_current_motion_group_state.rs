use crate::objects::motion_group_state_response::MotionGroupStateResponse;
use reqwest;

pub enum GetCurrentMotionGroupStateResponseType {
    Ok(MotionGroupStateResponse),
    UndefinedResponse(reqwest::Response),
}

pub struct GetCurrentMotionGroupStatePathParameters {
    pub motion_group: String,
    pub cell: String,
}
pub struct GetCurrentMotionGroupStateQueryParameters {
    pub response_coordinate_system: Option<String>,
    pub tcp: Option<String>,
}

pub async fn get_current_motion_group_state(
    client: &reqwest::Client,
    server: &str,
    path_parameters: &GetCurrentMotionGroupStatePathParameters,
    query_parameters: &GetCurrentMotionGroupStateQueryParameters,
) -> Result<GetCurrentMotionGroupStateResponseType, reqwest::Error> {
    let mut request_query_parameters: Vec<(&str, String)> = vec![];
    if let Some(ref query_parameter) = query_parameters.response_coordinate_system {
        request_query_parameters.push(("response_coordinate_system", query_parameter.to_string()));
    }
    if let Some(ref query_parameter) = query_parameters.tcp {
        request_query_parameters.push(("tcp", query_parameter.to_string()));
    }
    let response = match client
        .get(format!(
            "{server}/cells/{}/motion-groups/{}/state",
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
        200 => match response.json::<MotionGroupStateResponse>().await {
            Ok(motion_group_state_response) => Ok(GetCurrentMotionGroupStateResponseType::Ok(
                motion_group_state_response,
            )),
            Err(parsing_error) => Err(parsing_error),
        },
        _ => Ok(GetCurrentMotionGroupStateResponseType::UndefinedResponse(
            response,
        )),
    }
}
