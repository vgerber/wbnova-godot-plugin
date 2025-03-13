use crate::objects::http_exception_response::HttpExceptionResponse;
use crate::objects::http_validation_error::HttpValidationError;
use crate::objects::program_metadata::ProgramMetadata;
use reqwest;

pub enum NotFoundValue {
    Json(HttpExceptionResponse),
    Text(String),
}

pub enum CreateProgramResponseType {
    NotFound(NotFoundValue),
    Ok(ProgramMetadata),
    UnprocessableEntity(HttpValidationError),
    UndefinedResponse(reqwest::Response),
}

pub struct CreateProgramPathParameters {
    pub cell: String,
}
pub struct CreateProgramQueryParameters {
    pub name: Option<String>,
}

pub async fn create_program(
    client: &reqwest::Client,
    server: &str,
    content: &String,
    path_parameters: &CreateProgramPathParameters,
    query_parameters: &CreateProgramQueryParameters,
) -> Result<CreateProgramResponseType, reqwest::Error> {
    let mut request_query_parameters: Vec<(&str, String)> = vec![];
    if let Some(ref query_parameter) = query_parameters.name {
        request_query_parameters.push(("name", query_parameter.to_string()));
    }
    let body = content.to_owned();
    let response = match client
        .post(format!(
            "{server}/cells/{}/store/programs",
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
    let content_type = match response.headers().get("content-type") {
        Some(content_type) => match content_type.to_str() {
            Ok(content_type) => content_type,
            Err(_) => "text/plain",
        },
        None => return Ok(CreateProgramResponseType::UndefinedResponse(response)),
    };

    match response.status().as_u16() {
        404 => match content_type {
            "application/json" => match response.json::<HttpExceptionResponse>().await {
                Ok(http_exception_response) => Ok(CreateProgramResponseType::NotFound(
                    NotFoundValue::Json(http_exception_response),
                )),
                Err(parsing_error) => Err(parsing_error),
            },
            "text/plain" => match response.text().await {
                Ok(response_text) => Ok(CreateProgramResponseType::NotFound(NotFoundValue::Text(
                    response_text,
                ))),
                Err(parsing_error) => Err(parsing_error),
            },
            _ => Ok(CreateProgramResponseType::UndefinedResponse(response)),
        },
        200 => match response.json::<ProgramMetadata>().await {
            Ok(program_metadata) => Ok(CreateProgramResponseType::Ok(program_metadata)),
            Err(parsing_error) => Err(parsing_error),
        },
        422 => match response.json::<HttpValidationError>().await {
            Ok(http_validation_error) => Ok(CreateProgramResponseType::UnprocessableEntity(
                http_validation_error,
            )),
            Err(parsing_error) => Err(parsing_error),
        },
        _ => Ok(CreateProgramResponseType::UndefinedResponse(response)),
    }
}
