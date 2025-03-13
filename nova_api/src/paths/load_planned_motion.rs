use crate::objects::plan_response::PlanResponse;
use crate::objects::planned_motion::PlannedMotion;
use reqwest;

pub enum LoadPlannedMotionResponseType {
    Ok(PlanResponse),
    UndefinedResponse(reqwest::Response),
}

pub struct LoadPlannedMotionPathParameters {
    pub cell: String,
}

pub async fn load_planned_motion(
    client: &reqwest::Client,
    server: &str,
    content: PlannedMotion,
    path_parameters: &LoadPlannedMotionPathParameters,
) -> Result<LoadPlannedMotionResponseType, reqwest::Error> {
    let request_query_parameters: Vec<(&str, String)> = vec![];
    let response = match client
        .post(format!(
            "{server}/cells/{}/planned-motions",
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
        200 => match response.json::<PlanResponse>().await {
            Ok(plan_response) => Ok(LoadPlannedMotionResponseType::Ok(plan_response)),
            Err(parsing_error) => Err(parsing_error),
        },
        _ => Ok(LoadPlannedMotionResponseType::UndefinedResponse(response)),
    }
}
