use crate::objects::motion_group_instance::MotionGroupInstance;
use reqwest;

pub enum ActivateMotionGroupResponseType {
    Ok(MotionGroupInstance),
    UndefinedResponse(reqwest::Response),
}

pub struct ActivateMotionGroupPathParameters {
    pub cell: String,
}
pub struct ActivateMotionGroupQueryParameters {
    pub motion_group: String,
}

pub async fn activate_motion_group(
    client: &reqwest::Client,
    server: &str,
    path_parameters: &ActivateMotionGroupPathParameters,
    query_parameters: &ActivateMotionGroupQueryParameters,
) -> Result<ActivateMotionGroupResponseType, reqwest::Error> {
    let request_query_parameters: Vec<(&str, String)> =
        vec![("motion_group", query_parameters.motion_group.to_string())];
    let response = match client
        .post(format!(
            "{server}/cells/{}/motion-groups",
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
        200 => match response.json::<MotionGroupInstance>().await {
            Ok(motion_group_instance) => {
                Ok(ActivateMotionGroupResponseType::Ok(motion_group_instance))
            }
            Err(parsing_error) => Err(parsing_error),
        },
        _ => Ok(ActivateMotionGroupResponseType::UndefinedResponse(response)),
    }
}
