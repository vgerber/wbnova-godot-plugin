use reqwest;

pub enum StoreObjectResponseType {
    UndefinedResponse(reqwest::Response),
}

pub struct StoreObjectPathParameters {
    pub key: String,
    pub cell: String,
}

pub async fn store_object(
    client: &reqwest::Client,
    server: &str,
    path_parameters: &StoreObjectPathParameters,
) -> Result<StoreObjectResponseType, reqwest::Error> {
    let request_query_parameters: Vec<(&str, String)> = vec![];
    let response = match client
        .put(format!(
            "{server}/cells/{}/store/objects/{}",
            path_parameters.key, path_parameters.cell
        ))
        .query(&request_query_parameters)
        .send()
        .await
    {
        Ok(response) => response,
        Err(err) => return Err(err),
    };
    match response.status().as_u16() {
        _ => Ok(StoreObjectResponseType::UndefinedResponse(response)),
    }
}
