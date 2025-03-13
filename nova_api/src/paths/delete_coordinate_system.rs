use reqwest;

pub enum DeleteCoordinateSystemResponseType {
    UndefinedResponse(reqwest::Response),
}

pub struct DeleteCoordinateSystemPathParameters {
    pub coordinate_system: String,
    pub cell: String,
}

pub async fn delete_coordinate_system(
    client: &reqwest::Client,
    server: &str,
    path_parameters: &DeleteCoordinateSystemPathParameters,
) -> Result<DeleteCoordinateSystemResponseType, reqwest::Error> {
    let request_query_parameters: Vec<(&str, String)> = vec![];
    let response = match client
        .delete(format!(
            "{server}/cells/{}/coordinate-systems/{}",
            path_parameters.coordinate_system, path_parameters.cell
        ))
        .query(&request_query_parameters)
        .send()
        .await
    {
        Ok(response) => response,
        Err(err) => return Err(err),
    };
    match response.status().as_u16() {
        _ => Ok(DeleteCoordinateSystemResponseType::UndefinedResponse(
            response,
        )),
    }
}
