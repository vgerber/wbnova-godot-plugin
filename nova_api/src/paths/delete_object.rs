use reqwest;

pub enum DeleteObjectResponseType {
    UndefinedResponse(reqwest::Response),
}

pub struct DeleteObjectPathParameters {
    pub cell: String,
    pub key: String,
}

pub async fn delete_object(
    client: &reqwest::Client,
    server: &str,
    path_parameters: &DeleteObjectPathParameters,
) -> Result<DeleteObjectResponseType, reqwest::Error> {
    let request_query_parameters: Vec<(&str, String)> = vec![];
    let response = match client
        .delete(format!(
            "{server}/cells/{}/store/objects/{}",
            path_parameters.cell, path_parameters.key
        ))
        .query(&request_query_parameters)
        .send()
        .await
    {
        Ok(response) => response,
        Err(err) => return Err(err),
    };
    match response.status().as_u16() {
        _ => Ok(DeleteObjectResponseType::UndefinedResponse(response)),
    }
}
