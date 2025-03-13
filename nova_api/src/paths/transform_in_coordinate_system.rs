use crate::objects::pose::Pose;
use reqwest;

pub enum TransformInCoordinateSystemResponseType {
    Ok(Pose),
    UndefinedResponse(reqwest::Response),
}

pub struct TransformInCoordinateSystemPathParameters {
    pub coordinate_system: String,
    pub cell: String,
}

pub async fn transform_in_coordinate_system(
    client: &reqwest::Client,
    server: &str,
    content: Pose,
    path_parameters: &TransformInCoordinateSystemPathParameters,
) -> Result<TransformInCoordinateSystemResponseType, reqwest::Error> {
    let request_query_parameters: Vec<(&str, String)> = vec![];
    let response = match client
        .post(format!(
            "{server}/cells/{}/coordinate-systems/{}/transform",
            path_parameters.coordinate_system, path_parameters.cell
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
        200 => match response.json::<Pose>().await {
            Ok(pose) => Ok(TransformInCoordinateSystemResponseType::Ok(pose)),
            Err(parsing_error) => Err(parsing_error),
        },
        _ => Ok(TransformInCoordinateSystemResponseType::UndefinedResponse(
            response,
        )),
    }
}
