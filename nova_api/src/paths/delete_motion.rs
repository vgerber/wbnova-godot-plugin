use reqwest;

pub enum DeleteMotionResponseType {
    UndefinedResponse(reqwest::Response),
}

pub struct DeleteMotionPathParameters {
    pub cell: String,
    pub motion: String,
}

pub async fn delete_motion(
    client: &reqwest::Client,
    server: &str,
    path_parameters: &DeleteMotionPathParameters,
) -> Result<DeleteMotionResponseType, reqwest::Error> {
    let request_query_parameters: Vec<(&str, String)> = vec![];
    let response = match client
        .delete(format!(
            "{server}/cells/{}/motions/{}/delete",
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
        _ => Ok(DeleteMotionResponseType::UndefinedResponse(response)),
    }
}
