use crate::objects::http_exception_response::HttpExceptionResponse;
use crate::objects::http_validation_error::HttpValidationError;
use crate::objects::response_getrecipe::ResponseGetrecipe;
use reqwest;

pub enum GetRecipeResponseType {
    NotFound(HttpExceptionResponse),
    Ok(ResponseGetrecipe),
    UnprocessableEntity(HttpValidationError),
    UndefinedResponse(reqwest::Response),
}

pub struct GetRecipePathParameters {
    pub recipe: String,
    pub cell: String,
}

pub async fn get_recipe(
    client: &reqwest::Client,
    server: &str,
    path_parameters: &GetRecipePathParameters,
) -> Result<GetRecipeResponseType, reqwest::Error> {
    let request_query_parameters: Vec<(&str, String)> = vec![];
    let response = match client
        .get(format!(
            "{server}/cells/{}/store/recipes/{}",
            path_parameters.recipe, path_parameters.cell
        ))
        .query(&request_query_parameters)
        .send()
        .await
    {
        Ok(response) => response,
        Err(err) => return Err(err),
    };
    match response.status().as_u16() {
        404 => match response.json::<HttpExceptionResponse>().await {
            Ok(http_exception_response) => {
                Ok(GetRecipeResponseType::NotFound(http_exception_response))
            }
            Err(parsing_error) => Err(parsing_error),
        },
        200 => match response.json::<ResponseGetrecipe>().await {
            Ok(response_getrecipe) => Ok(GetRecipeResponseType::Ok(response_getrecipe)),
            Err(parsing_error) => Err(parsing_error),
        },
        422 => match response.json::<HttpValidationError>().await {
            Ok(http_validation_error) => Ok(GetRecipeResponseType::UnprocessableEntity(
                http_validation_error,
            )),
            Err(parsing_error) => Err(parsing_error),
        },
        _ => Ok(GetRecipeResponseType::UndefinedResponse(response)),
    }
}
