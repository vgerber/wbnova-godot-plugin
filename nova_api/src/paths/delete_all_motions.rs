use reqwest;

pub enum DeleteAllMotionsResponseType {
    UndefinedResponse(reqwest::Response),
}

pub struct DeleteAllMotionsPathParameters {
    pub cell: String,
}

pub async fn delete_all_motions(
    client: &reqwest::Client,
    server: &str,
    path_parameters: &DeleteAllMotionsPathParameters,
) -> Result<DeleteAllMotionsResponseType, reqwest::Error> {
    let request_query_parameters: Vec<(&str, String)> = vec![];
    let response = match client
        .delete(format!("{server}/cells/{}/motions", path_parameters.cell))
        .query(&request_query_parameters)
        .send()
        .await
    {
        Ok(response) => response,
        Err(err) => return Err(err),
    };
    match response.status().as_u16() {
        _ => Ok(DeleteAllMotionsResponseType::UndefinedResponse(response)),
    }
}
