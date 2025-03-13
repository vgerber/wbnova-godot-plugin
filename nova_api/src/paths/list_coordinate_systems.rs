use crate::objects::list_response::ListResponse;
use reqwest;

pub enum ListCoordinateSystemsResponseType {
    Ok(ListResponse),
    UndefinedResponse(reqwest::Response),
}

pub struct ListCoordinateSystemsPathParameters {
    pub cell: String,
}
pub struct ListCoordinateSystemsQueryParameters {
    pub rotation_type: Option<String>,
}

pub async fn list_coordinate_systems(
    client: &reqwest::Client,
    server: &str,
    path_parameters: &ListCoordinateSystemsPathParameters,
    query_parameters: &ListCoordinateSystemsQueryParameters,
) -> Result<ListCoordinateSystemsResponseType, reqwest::Error> {
    let mut request_query_parameters: Vec<(&str, String)> = vec![];
    if let Some(ref query_parameter) = query_parameters.rotation_type {
        request_query_parameters.push(("rotation_type", query_parameter.to_string()));
    }
    let response = match client
        .get(format!(
            "{server}/cells/{}/coordinate-systems",
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
        200 => match response.json::<ListResponse>().await {
            Ok(list_response) => Ok(ListCoordinateSystemsResponseType::Ok(list_response)),
            Err(parsing_error) => Err(parsing_error),
        },
        _ => Ok(ListCoordinateSystemsResponseType::UndefinedResponse(
            response,
        )),
    }
}
