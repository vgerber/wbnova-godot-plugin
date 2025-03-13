use crate::objects::coordinate_system::CoordinateSystem;
use reqwest;

pub enum GetCoordinateSystemResponseType {
    Ok(CoordinateSystem),
    UndefinedResponse(reqwest::Response),
}

pub struct GetCoordinateSystemPathParameters {
    pub cell: String,
    pub coordinate_system: String,
}
pub struct GetCoordinateSystemQueryParameters {
    pub rotation_type: Option<String>,
}

pub async fn get_coordinate_system(
    client: &reqwest::Client,
    server: &str,
    path_parameters: &GetCoordinateSystemPathParameters,
    query_parameters: &GetCoordinateSystemQueryParameters,
) -> Result<GetCoordinateSystemResponseType, reqwest::Error> {
    let mut request_query_parameters: Vec<(&str, String)> = vec![];
    if let Some(ref query_parameter) = query_parameters.rotation_type {
        request_query_parameters.push(("rotation_type", query_parameter.to_string()));
    }
    let response = match client
        .get(format!(
            "{server}/cells/{}/coordinate-systems/{}",
            path_parameters.cell, path_parameters.coordinate_system
        ))
        .query(&request_query_parameters)
        .send()
        .await
    {
        Ok(response) => response,
        Err(err) => return Err(err),
    };
    match response.status().as_u16() {
        200 => match response.json::<CoordinateSystem>().await {
            Ok(coordinate_system) => Ok(GetCoordinateSystemResponseType::Ok(coordinate_system)),
            Err(parsing_error) => Err(parsing_error),
        },
        _ => Ok(GetCoordinateSystemResponseType::UndefinedResponse(response)),
    }
}
