use crate::objects::add_request::AddRequest;
use crate::objects::coordinate_system::CoordinateSystem;
use reqwest;

pub enum AddCoordinateSystemResponseType {
    Ok(CoordinateSystem),
    UndefinedResponse(reqwest::Response),
}

pub struct AddCoordinateSystemPathParameters {
    pub cell: String,
}

pub async fn add_coordinate_system(
    client: &reqwest::Client,
    server: &str,
    content: AddRequest,
    path_parameters: &AddCoordinateSystemPathParameters,
) -> Result<AddCoordinateSystemResponseType, reqwest::Error> {
    let request_query_parameters: Vec<(&str, String)> = vec![];
    let response = match client
        .put(format!(
            "{server}/cells/{}/coordinate-systems",
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
        200 => match response.json::<CoordinateSystem>().await {
            Ok(coordinate_system) => Ok(AddCoordinateSystemResponseType::Ok(coordinate_system)),
            Err(parsing_error) => Err(parsing_error),
        },
        _ => Ok(AddCoordinateSystemResponseType::UndefinedResponse(response)),
    }
}
