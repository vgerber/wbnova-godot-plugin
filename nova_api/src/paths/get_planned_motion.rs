use crate::objects::planned_motion::PlannedMotion;
use reqwest;

pub enum GetPlannedMotionResponseType {
    Ok(PlannedMotion),
    UndefinedResponse(reqwest::Response),
}

pub struct GetPlannedMotionPathParameters {
    pub cell: String,
    pub motion: String,
}
pub struct GetPlannedMotionQueryParameters {
    pub sample_time: Option<i32>,
}

pub async fn get_planned_motion(
    client: &reqwest::Client,
    server: &str,
    path_parameters: &GetPlannedMotionPathParameters,
    query_parameters: &GetPlannedMotionQueryParameters,
) -> Result<GetPlannedMotionResponseType, reqwest::Error> {
    let mut request_query_parameters: Vec<(&str, String)> = vec![];
    if let Some(ref query_parameter) = query_parameters.sample_time {
        request_query_parameters.push(("sample_time", query_parameter.to_string()));
    }
    let response = match client
        .get(format!(
            "{server}/cells/{}/motions/{}/planned-motion",
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
        200 => match response.json::<PlannedMotion>().await {
            Ok(planned_motion) => Ok(GetPlannedMotionResponseType::Ok(planned_motion)),
            Err(parsing_error) => Err(parsing_error),
        },
        _ => Ok(GetPlannedMotionResponseType::UndefinedResponse(response)),
    }
}
