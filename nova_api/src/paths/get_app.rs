use crate::objects::app::App;
use crate::objects::error::Error;
use reqwest;

pub enum GetAppResponseType {
    Ok(App),
    NotFound(Error),
    UndefinedResponse(reqwest::Response),
}

pub struct GetAppPathParameters {
    pub app: String,
    pub cell: String,
}

pub async fn get_app(
    client: &reqwest::Client,
    server: &str,
    path_parameters: &GetAppPathParameters,
) -> Result<GetAppResponseType, reqwest::Error> {
    let request_query_parameters: Vec<(&str, String)> = vec![];
    let response = match client
        .get(format!(
            "{server}/cells/{}/apps/{}",
            path_parameters.app, path_parameters.cell
        ))
        .query(&request_query_parameters)
        .send()
        .await
    {
        Ok(response) => response,
        Err(err) => return Err(err),
    };
    match response.status().as_u16() {
        200 => match response.json::<App>().await {
            Ok(app) => Ok(GetAppResponseType::Ok(app)),
            Err(parsing_error) => Err(parsing_error),
        },
        404 => match response.json::<Error>().await {
            Ok(error) => Ok(GetAppResponseType::NotFound(error)),
            Err(parsing_error) => Err(parsing_error),
        },
        _ => Ok(GetAppResponseType::UndefinedResponse(response)),
    }
}
