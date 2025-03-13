use reqwest;

pub enum ClearAllObjectsResponseType {
    UndefinedResponse(reqwest::Response),
}

pub struct ClearAllObjectsPathParameters {
    pub cell: String,
}

pub async fn clear_all_objects(
    client: &reqwest::Client,
    server: &str,
    path_parameters: &ClearAllObjectsPathParameters,
) -> Result<ClearAllObjectsResponseType, reqwest::Error> {
    let request_query_parameters: Vec<(&str, String)> = vec![];
    let response = match client
        .delete(format!(
            "{server}/cells/{}/store/objects",
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
        _ => Ok(ClearAllObjectsResponseType::UndefinedResponse(response)),
    }
}
