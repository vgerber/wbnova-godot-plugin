use reqwest;

pub enum GetObjectResponseType {
    UndefinedResponse(reqwest::Response),
}

pub struct GetObjectPathParameters {
    pub cell: String,
    pub key: String,
}

pub async fn get_object(
    client: &reqwest::Client,
    server: &str,
    path_parameters: &GetObjectPathParameters,
) -> Result<GetObjectResponseType, reqwest::Error> {
    let request_query_parameters: Vec<(&str, String)> = vec![];
    let response = match client
        .get(format!(
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
        _ => Ok(GetObjectResponseType::UndefinedResponse(response)),
    }
}
