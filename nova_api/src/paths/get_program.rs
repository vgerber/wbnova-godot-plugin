use crate::objects::http_exception_response::HttpExceptionResponse;
use crate::objects::http_validation_error::HttpValidationError;
use reqwest;

pub enum NotFoundValue {
    Json(HttpExceptionResponse),
    Text(String),
}

pub enum GetProgramResponseType {
    UnprocessableEntity(HttpValidationError),
    NotFound(NotFoundValue),
    Ok(String),
    UndefinedResponse(reqwest::Response),
}

pub struct GetProgramPathParameters {
    pub cell: String,
    pub program: String,
}

pub async fn get_program(
    client: &reqwest::Client,
    server: &str,
    path_parameters: &GetProgramPathParameters,
) -> Result<GetProgramResponseType, reqwest::Error> {
    let request_query_parameters: Vec<(&str, String)> = vec![];
    let response = match client
        .get(format!(
            "{server}/cells/{}/store/programs/{}",
            path_parameters.cell, path_parameters.program
        ))
        .query(&request_query_parameters)
        .send()
        .await
    {
        Ok(response) => response,
        Err(err) => return Err(err),
    };
    let content_type = match response.headers().get("content-type") {
        Some(content_type) => match content_type.to_str() {
            Ok(content_type) => content_type,
            Err(_) => "text/plain",
        },
        None => return Ok(GetProgramResponseType::UndefinedResponse(response)),
    };

    match response.status().as_u16() {
        422 => match response.json::<HttpValidationError>().await {
            Ok(http_validation_error) => Ok(GetProgramResponseType::UnprocessableEntity(
                http_validation_error,
            )),
            Err(parsing_error) => Err(parsing_error),
        },
        404 => match content_type {
            "application/json" => match response.json::<HttpExceptionResponse>().await {
                Ok(http_exception_response) => Ok(GetProgramResponseType::NotFound(
                    NotFoundValue::Json(http_exception_response),
                )),
                Err(parsing_error) => Err(parsing_error),
            },
            "text/plain" => match response.text().await {
                Ok(response_text) => Ok(GetProgramResponseType::NotFound(NotFoundValue::Text(
                    response_text,
                ))),
                Err(parsing_error) => Err(parsing_error),
            },
            _ => Ok(GetProgramResponseType::UndefinedResponse(response)),
        },
        200 => match response.text().await {
            Ok(response_text) => Ok(GetProgramResponseType::Ok(response_text)),
            Err(parsing_error) => Err(parsing_error),
        },
        _ => Ok(GetProgramResponseType::UndefinedResponse(response)),
    }
}
