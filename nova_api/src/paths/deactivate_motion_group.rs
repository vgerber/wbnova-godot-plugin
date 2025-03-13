use reqwest;

pub enum DeactivateMotionGroupResponseType {
    UndefinedResponse(reqwest::Response),
}

pub struct DeactivateMotionGroupPathParameters {
    pub motion_group: String,
    pub cell: String,
}

pub async fn deactivate_motion_group(
    client: &reqwest::Client,
    server: &str,
    path_parameters: &DeactivateMotionGroupPathParameters,
) -> Result<DeactivateMotionGroupResponseType, reqwest::Error> {
    let request_query_parameters: Vec<(&str, String)> = vec![];
    let response = match client
        .delete(format!(
            "{server}/cells/{}/motion-groups/{}",
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
        _ => Ok(DeactivateMotionGroupResponseType::UndefinedResponse(
            response,
        )),
    }
}
