use crate::objects::http_validation_error::HttpValidationError;
use crate::objects::plan_trajectory_request::PlanTrajectoryRequest;
use crate::objects::plan_trajectory_response::PlanTrajectoryResponse;
use reqwest;

pub enum PlanCollisionFreePtpResponseType {
    Ok(PlanTrajectoryResponse),
    UnprocessableEntity(HttpValidationError),
    UndefinedResponse(reqwest::Response),
}

pub struct PlanCollisionFreePtpPathParameters {
    pub cell: String,
}

pub async fn plan_collision_free_ptp(
    client: &reqwest::Client,
    server: &str,
    content: PlanTrajectoryRequest,
    path_parameters: &PlanCollisionFreePtpPathParameters,
) -> Result<PlanCollisionFreePtpResponseType, reqwest::Error> {
    let request_query_parameters: Vec<(&str, String)> = vec![];
    let response = match client
        .post(format!(
            "{server}/cells/{}/motion-planning/plan-collision-free-ptp",
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
        200 => match response.json::<PlanTrajectoryResponse>().await {
            Ok(plan_trajectory_response) => Ok(PlanCollisionFreePtpResponseType::Ok(
                plan_trajectory_response,
            )),
            Err(parsing_error) => Err(parsing_error),
        },
        422 => match response.json::<HttpValidationError>().await {
            Ok(http_validation_error) => Ok(PlanCollisionFreePtpResponseType::UnprocessableEntity(
                http_validation_error,
            )),
            Err(parsing_error) => Err(parsing_error),
        },
        _ => Ok(PlanCollisionFreePtpResponseType::UndefinedResponse(
            response,
        )),
    }
}
