use crate::objects::http_exception_response::HttpExceptionResponse;
use crate::objects::http_validation_error::HttpValidationError;
use crate::objects::list_program_metadata_response::ListProgramMetadataResponse;
use reqwest;

pub enum NotFoundValue {
    Json(HttpExceptionResponse),
    Text(String),
}

pub enum DeleteProgramListResponseType {
    Ok(ListProgramMetadataResponse),
    UnprocessableEntity(HttpValidationError),
    NotFound(NotFoundValue),
    UndefinedResponse(reqwest::Response),
}

pub struct DeleteProgramListPathParameters {
    pub cell: String,
}
pub struct DeleteProgramListQueryParameters {
    pub program_ids: Vec<String>,
}

pub async fn delete_program_list(
    client: &reqwest::Client,
    server: &str,
    path_parameters: &DeleteProgramListPathParameters,
    query_parameters: &DeleteProgramListQueryParameters,
) -> Result<DeleteProgramListResponseType, reqwest::Error> {
    let mut request_query_parameters: Vec<(&str, String)> = vec![];
    query_parameters
        .program_ids
        .iter()
        .for_each(|query_parameter_item| {
            request_query_parameters.push(("program_ids", query_parameter_item.to_string()))
        });
    let response = match client
        .delete(format!(
            "{server}/cells/{}/store/programs",
            path_parameters.cell
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
        None => return Ok(DeleteProgramListResponseType::UndefinedResponse(response)),
    };

    match response.status().as_u16() {
        200 => match response.json::<ListProgramMetadataResponse>().await {
            Ok(list_program_metadata_response) => Ok(DeleteProgramListResponseType::Ok(
                list_program_metadata_response,
            )),
            Err(parsing_error) => Err(parsing_error),
        },
        422 => match response.json::<HttpValidationError>().await {
            Ok(http_validation_error) => Ok(DeleteProgramListResponseType::UnprocessableEntity(
                http_validation_error,
            )),
            Err(parsing_error) => Err(parsing_error),
        },
        404 => match content_type {
            "application/json" => match response.json::<HttpExceptionResponse>().await {
                Ok(http_exception_response) => Ok(DeleteProgramListResponseType::NotFound(
                    NotFoundValue::Json(http_exception_response),
                )),
                Err(parsing_error) => Err(parsing_error),
            },
            "text/plain" => match response.text().await {
                Ok(response_text) => Ok(DeleteProgramListResponseType::NotFound(
                    NotFoundValue::Text(response_text),
                )),
                Err(parsing_error) => Err(parsing_error),
            },
            _ => Ok(DeleteProgramListResponseType::UndefinedResponse(response)),
        },
        _ => Ok(DeleteProgramListResponseType::UndefinedResponse(response)),
    }
}
