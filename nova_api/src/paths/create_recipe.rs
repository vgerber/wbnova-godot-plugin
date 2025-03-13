use crate::objects::http_exception_response::HttpExceptionResponse;
use crate::objects::http_validation_error::HttpValidationError;
use crate::objects::recipe_metadata::RecipeMetadata;
use crate::objects::recipe_properties::RecipeProperties;
use reqwest;

pub enum CreateRecipeResponseType {
    UnprocessableEntity(HttpValidationError),
    NotFound(HttpExceptionResponse),
    Ok(RecipeMetadata),
    UndefinedResponse(reqwest::Response),
}

pub struct CreateRecipePathParameters {
    pub cell: String,
}
pub struct CreateRecipeQueryParameters {
    pub program_id: String,
    pub name: Option<String>,
}

pub async fn create_recipe(
    client: &reqwest::Client,
    server: &str,
    content: RecipeProperties,
    path_parameters: &CreateRecipePathParameters,
    query_parameters: &CreateRecipeQueryParameters,
) -> Result<CreateRecipeResponseType, reqwest::Error> {
    let mut request_query_parameters: Vec<(&str, String)> =
        vec![("program_id", query_parameters.program_id.to_string())];
    if let Some(ref query_parameter) = query_parameters.name {
        request_query_parameters.push(("name", query_parameter.to_string()));
    }
    let response = match client
        .post(format!(
            "{server}/cells/{}/store/recipes",
            path_parameters.cell
        ))
        .query(&request_query_parameters)
        .json(&content)
        .send()
        .await
    {
        Ok(response) => response,
        Err(err) => return Err(err),
    };
    match response.status().as_u16() {
        422 => match response.json::<HttpValidationError>().await {
            Ok(http_validation_error) => Ok(CreateRecipeResponseType::UnprocessableEntity(
                http_validation_error,
            )),
            Err(parsing_error) => Err(parsing_error),
        },
        404 => match response.json::<HttpExceptionResponse>().await {
            Ok(http_exception_response) => {
                Ok(CreateRecipeResponseType::NotFound(http_exception_response))
            }
            Err(parsing_error) => Err(parsing_error),
        },
        200 => match response.json::<RecipeMetadata>().await {
            Ok(recipe_metadata) => Ok(CreateRecipeResponseType::Ok(recipe_metadata)),
            Err(parsing_error) => Err(parsing_error),
        },
        _ => Ok(CreateRecipeResponseType::UndefinedResponse(response)),
    }
}
