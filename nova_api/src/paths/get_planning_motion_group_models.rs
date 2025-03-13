use crate::objects::http_validation_error::HttpValidationError;
use reqwest;

pub enum GetPlanningMotionGroupModelsResponseType {
    Ok(Vec<String>),
    UnprocessableEntity(HttpValidationError),
    UndefinedResponse(reqwest::Response),
}

pub struct GetPlanningMotionGroupModelsPathParameters {
    pub cell: String,
}

pub async fn get_planning_motion_group_models(
    client: &reqwest::Client,
    server: &str,
    path_parameters: &GetPlanningMotionGroupModelsPathParameters,
) -> Result<GetPlanningMotionGroupModelsResponseType, reqwest::Error> {
    let request_query_parameters: Vec<(&str, String)> = vec![];
    let response = match client
        .get(format!(
            "{server}/cells/{}/motion-planning/motion-group-models",
            path_parameters.cell
        ))
        .query(&request_query_parameters)
        .send()
        .await
    {
        Ok(response) => response,
        Err(err) => return Err(err),
    };
    match response.status().as_u16() {
        200 => match response.json::<Vec<String>>().await {
            Ok(models) => Ok(GetPlanningMotionGroupModelsResponseType::Ok(models)),
            Err(parsing_error) => Err(parsing_error),
        },
        422 => match response.json::<HttpValidationError>().await {
            Ok(http_validation_error) => Ok(
                GetPlanningMotionGroupModelsResponseType::UnprocessableEntity(
                    http_validation_error,
                ),
            ),
            Err(parsing_error) => Err(parsing_error),
        },
        _ => Ok(GetPlanningMotionGroupModelsResponseType::UndefinedResponse(
            response,
        )),
    }
}
