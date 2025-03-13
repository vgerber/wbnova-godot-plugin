use crate::objects::http_exception_response::HttpExceptionResponse;
use crate::objects::http_validation_error::HttpValidationError;
use crate::objects::recipe_metadata::RecipeMetadata;
use crate::objects::updated_recipe_properties::UpdatedRecipeProperties;
use reqwest;

pub enum UpdateRecipeResponseType {
    UnprocessableEntity(HttpValidationError),
    Ok(RecipeMetadata),
    NotFound(HttpExceptionResponse),
    UndefinedResponse(reqwest::Response),
}

pub struct UpdateRecipePathParameters {
    pub cell: String,
    pub recipe: String,
}

pub async fn update_recipe(
    client: &reqwest::Client,
    server: &str,
    content: UpdatedRecipeProperties,
    path_parameters: &UpdateRecipePathParameters,
) -> Result<UpdateRecipeResponseType, reqwest::Error> {
    let request_query_parameters: Vec<(&str, String)> = vec![];
    let response = match client
        .put(format!(
            "{server}/cells/{}/store/recipes/{}",
            path_parameters.cell, path_parameters.recipe
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
            Ok(http_validation_error) => Ok(UpdateRecipeResponseType::UnprocessableEntity(
                http_validation_error,
            )),
            Err(parsing_error) => Err(parsing_error),
        },
        200 => match response.json::<RecipeMetadata>().await {
            Ok(recipe_metadata) => Ok(UpdateRecipeResponseType::Ok(recipe_metadata)),
            Err(parsing_error) => Err(parsing_error),
        },
        404 => match response.json::<HttpExceptionResponse>().await {
            Ok(http_exception_response) => {
                Ok(UpdateRecipeResponseType::NotFound(http_exception_response))
            }
            Err(parsing_error) => Err(parsing_error),
        },
        _ => Ok(UpdateRecipeResponseType::UndefinedResponse(response)),
    }
}
