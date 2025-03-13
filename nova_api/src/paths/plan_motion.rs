use crate::objects::plan_request::PlanRequest;
use crate::objects::plan_response::PlanResponse;
use reqwest;

pub enum PlanMotionResponseType {
    Ok(PlanResponse),
    UndefinedResponse(reqwest::Response),
}

pub struct PlanMotionPathParameters {
    pub cell: String,
}

pub async fn plan_motion(
    client: &reqwest::Client,
    server: &str,
    content: PlanRequest,
    path_parameters: &PlanMotionPathParameters,
) -> Result<PlanMotionResponseType, reqwest::Error> {
    let request_query_parameters: Vec<(&str, String)> = vec![];
    let response = match client
        .post(format!("{server}/cells/{}/motions", path_parameters.cell))
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
            Ok(plan_response) => Ok(PlanMotionResponseType::Ok(plan_response)),
            Err(parsing_error) => Err(parsing_error),
        },
        _ => Ok(PlanMotionResponseType::UndefinedResponse(response)),
    }
}
