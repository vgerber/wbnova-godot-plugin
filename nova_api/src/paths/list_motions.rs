use crate::objects::motion_ids_list_response::MotionIdsListResponse;
use reqwest;

pub enum ListMotionsResponseType {
    Ok(MotionIdsListResponse),
    UndefinedResponse(reqwest::Response),
}

pub struct ListMotionsPathParameters {
    pub cell: String,
}

pub async fn list_motions(
    client: &reqwest::Client,
    server: &str,
    path_parameters: &ListMotionsPathParameters,
) -> Result<ListMotionsResponseType, reqwest::Error> {
    let request_query_parameters: Vec<(&str, String)> = vec![];
    let response = match client
        .get(format!("{server}/cells/{}/motions", path_parameters.cell))
        .query(&request_query_parameters)
        .send()
        .await
    {
        Ok(response) => response,
        Err(err) => return Err(err),
    };
    match response.status().as_u16() {
        200 => match response.json::<MotionIdsListResponse>().await {
            Ok(motion_ids_list_response) => {
                Ok(ListMotionsResponseType::Ok(motion_ids_list_response))
            }
            Err(parsing_error) => Err(parsing_error),
        },
        _ => Ok(ListMotionsResponseType::UndefinedResponse(response)),
    }
}
