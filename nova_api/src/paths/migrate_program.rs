use crate::objects::http_validation_error::HttpValidationError;
use reqwest;

pub enum MigrateProgramResponseType {
    Ok(String),
    UnprocessableEntity(HttpValidationError),
    UndefinedResponse(reqwest::Response),
}

pub struct MigrateProgramPathParameters {
    pub cell: String,
}

pub async fn migrate_program(
    client: &reqwest::Client,
    server: &str,
    content: &String,
    path_parameters: &MigrateProgramPathParameters,
) -> Result<MigrateProgramResponseType, reqwest::Error> {
    let request_query_parameters: Vec<(&str, String)> = vec![];
    let body = content.to_owned();
    let response = match client
        .post(format!(
            "{server}/cells/{}/programs/migrate",
            path_parameters.cell
        ))
        .query(&request_query_parameters)
        .body(body)
        .send()
        .await
    {
        Ok(response) => response,
        Err(err) => return Err(err),
    };
    match response.status().as_u16() {
        200 => match response.text().await {
            Ok(response_text) => Ok(MigrateProgramResponseType::Ok(response_text)),
            Err(parsing_error) => Err(parsing_error),
        },
        422 => match response.json::<HttpValidationError>().await {
            Ok(http_validation_error) => Ok(MigrateProgramResponseType::UnprocessableEntity(
                http_validation_error,
            )),
            Err(parsing_error) => Err(parsing_error),
        },
        _ => Ok(MigrateProgramResponseType::UndefinedResponse(response)),
    }
}
