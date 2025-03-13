use crate::objects::http_exception_response::HttpExceptionResponse;
use crate::objects::http_validation_error::HttpValidationError;
use crate::objects::program_metadata::ProgramMetadata;
use reqwest;

pub enum NotFoundValue {
    Json(HttpExceptionResponse),
    Text(String),
}

pub enum DeleteProgramResponseType {
    Ok(ProgramMetadata),
    NotFound(NotFoundValue),
    UnprocessableEntity(HttpValidationError),
    UndefinedResponse(reqwest::Response),
}

pub struct DeleteProgramPathParameters {
    pub program: String,
    pub cell: String,
}

pub async fn delete_program(
    client: &reqwest::Client,
    server: &str,
    path_parameters: &DeleteProgramPathParameters,
) -> Result<DeleteProgramResponseType, reqwest::Error> {
    let request_query_parameters: Vec<(&str, String)> = vec![];
    let response = match client
        .delete(format!(
            "{server}/cells/{}/store/programs/{}",
            path_parameters.program, path_parameters.cell
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
        None => return Ok(DeleteProgramResponseType::UndefinedResponse(response)),
    };

    match response.status().as_u16() {
        200 => match response.json::<ProgramMetadata>().await {
            Ok(program_metadata) => Ok(DeleteProgramResponseType::Ok(program_metadata)),
            Err(parsing_error) => Err(parsing_error),
        },
        404 => match content_type {
            "application/json" => match response.json::<HttpExceptionResponse>().await {
                Ok(http_exception_response) => Ok(DeleteProgramResponseType::NotFound(
                    NotFoundValue::Json(http_exception_response),
                )),
                Err(parsing_error) => Err(parsing_error),
            },
            "text/plain" => match response.text().await {
                Ok(response_text) => Ok(DeleteProgramResponseType::NotFound(NotFoundValue::Text(
                    response_text,
                ))),
                Err(parsing_error) => Err(parsing_error),
            },
            _ => Ok(DeleteProgramResponseType::UndefinedResponse(response)),
        },
        422 => match response.json::<HttpValidationError>().await {
            Ok(http_validation_error) => Ok(DeleteProgramResponseType::UnprocessableEntity(
                http_validation_error,
            )),
            Err(parsing_error) => Err(parsing_error),
        },
        _ => Ok(DeleteProgramResponseType::UndefinedResponse(response)),
    }
}
