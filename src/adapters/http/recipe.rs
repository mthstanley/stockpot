use serde_with::{self};
use std::sync::Arc;

use axum::{
    extract::{Path, State},
    http::StatusCode,
    routing::{delete, get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};

use crate::{adapters, core::domain};

use super::{error::AppError, extract::ExtractAuthUser, AppState};

#[derive(Serialize)]
pub struct GetUnit {
    pub id: i32,
    pub name: String,
}

impl From<domain::recipe::Unit> for GetUnit {
    fn from(value: domain::recipe::Unit) -> Self {
        Self {
            id: value.id.unwrap_or(-1),
            name: value.name,
        }
    }
}

#[derive(Serialize)]
pub struct GetIngredient {
    pub id: i32,
    pub name: String,
}

impl From<domain::recipe::Ingredient> for GetIngredient {
    fn from(value: domain::recipe::Ingredient) -> Self {
        Self {
            id: value.id.unwrap_or(-1),
            name: value.name,
        }
    }
}

#[derive(Serialize)]
pub struct GetRecipeIngredient {
    pub id: i32,
    pub recipe_id: i32,
    pub ingredient: GetIngredient,
    pub quantity: i32,
    pub units: GetUnit,
    pub preparation: String,
}

impl From<domain::recipe::RecipeIngredient> for GetRecipeIngredient {
    fn from(value: domain::recipe::RecipeIngredient) -> Self {
        Self {
            id: value.id.unwrap_or(-1),
            recipe_id: value.recipe_id.unwrap_or(-1),
            ingredient: value.ingredient.into(),
            quantity: value.quantity,
            units: value.units.into(),
            preparation: value.preparation,
        }
    }
}

#[derive(Serialize)]
pub struct GetStep {
    pub id: i32,
    pub recipe_id: i32,
    pub ordinal: i32,
    pub instruction: String,
}

impl From<domain::recipe::Step> for GetStep {
    fn from(value: domain::recipe::Step) -> Self {
        Self {
            id: value.id.unwrap_or(-1),
            recipe_id: value.recipe_id.unwrap_or(-1),
            ordinal: value.ordinal,
            instruction: value.instruction,
        }
    }
}

#[serde_with::serde_as]
#[derive(Serialize)]
pub struct GetRecipe {
    pub id: i32,
    pub title: String,
    pub description: Option<String>,
    pub author: adapters::http::user::GetUser,
    #[serde_as(as = "Option<serde_with::DurationSeconds<i64>>")]
    pub prep_time: Option<chrono::Duration>,
    #[serde_as(as = "Option<serde_with::DurationSeconds<i64>>")]
    pub cook_time: Option<chrono::Duration>,
    #[serde_as(as = "Option<serde_with::DurationSeconds<i64>>")]
    pub inactive_time: Option<chrono::Duration>,
    pub yield_quantity: i32,
    pub yield_units: GetUnit,
    pub ingredients: Vec<GetRecipeIngredient>,
    pub steps: Vec<GetStep>,
}

impl From<domain::Recipe> for GetRecipe {
    fn from(value: domain::Recipe) -> Self {
        Self {
            id: value.id.unwrap_or(-1),
            title: value.title,
            description: value.description,
            author: value.author.into(),
            prep_time: value.prep_time,
            cook_time: value.cook_time,
            inactive_time: value.inactive_time,
            yield_quantity: value.yield_quantity,
            yield_units: value.yield_units.into(),
            ingredients: value.ingredients.into_iter().map(|x| x.into()).collect(),
            steps: value.steps.into_iter().map(|x| x.into()).collect(),
        }
    }
}

#[derive(Deserialize)]
pub struct CreateUnit {
    pub name: String,
}

impl From<CreateUnit> for domain::recipe::Unit {
    fn from(value: CreateUnit) -> Self {
        Self {
            id: None,
            name: value.name,
        }
    }
}

#[derive(Deserialize)]
pub struct CreateIngredient {
    pub name: String,
}

impl From<CreateIngredient> for domain::recipe::Ingredient {
    fn from(value: CreateIngredient) -> Self {
        Self {
            id: None,
            name: value.name,
        }
    }
}

#[derive(Deserialize)]
pub struct CreateRecipeIngredient {
    pub ingredient: CreateIngredient,
    pub quantity: i32,
    pub units: CreateUnit,
    pub preparation: String,
}

impl From<CreateRecipeIngredient> for domain::recipe::RecipeIngredient {
    fn from(value: CreateRecipeIngredient) -> Self {
        Self {
            id: None,
            recipe_id: None,
            ingredient: value.ingredient.into(),
            quantity: value.quantity,
            units: value.units.into(),
            preparation: value.preparation,
        }
    }
}

#[derive(Deserialize)]
pub struct CreateStep {
    pub ordinal: i32,
    pub instruction: String,
}

impl From<CreateStep> for domain::recipe::Step {
    fn from(value: CreateStep) -> Self {
        Self {
            id: None,
            recipe_id: None,
            ordinal: value.ordinal,
            instruction: value.instruction,
        }
    }
}

#[serde_with::serde_as]
#[derive(Deserialize)]
pub struct CreateRecipe {
    pub title: String,
    pub description: Option<String>,
    #[serde_as(as = "Option<serde_with::DurationSeconds<i64>>")]
    pub prep_time: Option<chrono::Duration>,
    #[serde_as(as = "Option<serde_with::DurationSeconds<i64>>")]
    pub cook_time: Option<chrono::Duration>,
    #[serde_as(as = "Option<serde_with::DurationSeconds<i64>>")]
    pub inactive_time: Option<chrono::Duration>,
    pub yield_quantity: i32,
    pub yield_units: CreateUnit,
    pub ingredients: Vec<CreateRecipeIngredient>,
    pub steps: Vec<CreateStep>,
}

impl domain::Recipe {
    fn from_create(value: CreateRecipe, author: domain::User) -> Self {
        Self {
            id: None,
            title: value.title,
            description: value.description,
            author,
            prep_time: value.prep_time,
            cook_time: value.cook_time,
            inactive_time: value.inactive_time,
            yield_quantity: value.yield_quantity,
            yield_units: value.yield_units.into(),
            ingredients: value.ingredients.into_iter().map(|x| x.into()).collect(),
            steps: value.steps.into_iter().map(|x| x.into()).collect(),
        }
    }
}

#[derive(Deserialize)]
pub struct UpdateUnit {
    pub id: i32,
    pub name: String,
}

impl From<UpdateUnit> for domain::recipe::Unit {
    fn from(value: UpdateUnit) -> Self {
        Self {
            id: Some(value.id),
            name: value.name,
        }
    }
}

#[derive(Deserialize)]
pub struct UpdateIngredient {
    pub id: i32,
    pub name: String,
}

impl From<UpdateIngredient> for domain::recipe::Ingredient {
    fn from(value: UpdateIngredient) -> Self {
        Self {
            id: Some(value.id),
            name: value.name,
        }
    }
}

#[derive(Deserialize)]
pub struct UpdateRecipeIngredient {
    pub id: i32,
    pub recipe_id: i32,
    pub ingredient: UpdateIngredient,
    pub quantity: i32,
    pub units: UpdateUnit,
    pub preparation: String,
}

impl From<UpdateRecipeIngredient> for domain::recipe::RecipeIngredient {
    fn from(value: UpdateRecipeIngredient) -> Self {
        Self {
            id: Some(value.id),
            recipe_id: Some(value.recipe_id),
            ingredient: value.ingredient.into(),
            quantity: value.quantity,
            units: value.units.into(),
            preparation: value.preparation,
        }
    }
}

#[derive(Deserialize)]
pub struct UpdateStep {
    pub id: i32,
    pub recipe_id: i32,
    pub ordinal: i32,
    pub instruction: String,
}

impl From<UpdateStep> for domain::recipe::Step {
    fn from(value: UpdateStep) -> Self {
        Self {
            id: Some(value.id),
            recipe_id: Some(value.recipe_id),
            ordinal: value.ordinal,
            instruction: value.instruction,
        }
    }
}

#[serde_with::serde_as]
#[derive(Deserialize)]
pub struct UpdateRecipe {
    pub id: i32,
    pub title: String,
    pub description: Option<String>,
    #[serde_as(as = "Option<serde_with::DurationSeconds<i64>>")]
    pub prep_time: Option<chrono::Duration>,
    #[serde_as(as = "Option<serde_with::DurationSeconds<i64>>")]
    pub cook_time: Option<chrono::Duration>,
    #[serde_as(as = "Option<serde_with::DurationSeconds<i64>>")]
    pub inactive_time: Option<chrono::Duration>,
    pub yield_quantity: i32,
    pub yield_units: UpdateUnit,
    pub ingredients: Vec<UpdateRecipeIngredient>,
    pub steps: Vec<UpdateStep>,
}

impl domain::Recipe {
    fn from_update(value: UpdateRecipe, author: domain::User) -> Self {
        Self {
            id: Some(value.id),
            title: value.title,
            description: value.description,
            author,
            prep_time: value.prep_time,
            cook_time: value.cook_time,
            inactive_time: value.inactive_time,
            yield_quantity: value.yield_quantity,
            yield_units: value.yield_units.into(),
            ingredients: value.ingredients.into_iter().map(|x| x.into()).collect(),
            steps: value.steps.into_iter().map(|x| x.into()).collect(),
        }
    }
}

pub fn build_routes() -> Router<Arc<AppState>> {
    Router::new()
        .route("/recipe", get(get_recipes))
        .route("/recipe/:id", get(get_recipe))
        .route("/recipe", post(create_recipe))
        .route("/recipe/:id", post(update_recipe))
        .route("/recipe/:id", delete(delete_recipe))
}

pub async fn get_recipes(
    State(state): State<Arc<AppState>>,
) -> anyhow::Result<Json<Vec<GetRecipe>>, AppError> {
    Ok(Json(
        state
            .recipe_service
            .get_recipes()
            .await?
            .into_iter()
            .map(|x| x.into())
            .collect(),
    ))
}

pub async fn get_recipe(
    State(state): State<Arc<AppState>>,
    Path(id): Path<i32>,
) -> anyhow::Result<Json<GetRecipe>, AppError> {
    Ok(Json(
        state.recipe_service.get_recipe_by_id(id).await?.into(),
    ))
}

pub async fn create_recipe(
    State(state): State<Arc<AppState>>,
    ExtractAuthUser(auth_user): ExtractAuthUser,
    Json(recipe_request): Json<CreateRecipe>,
) -> anyhow::Result<(StatusCode, Json<GetRecipe>), AppError> {
    Ok((
        StatusCode::CREATED,
        Json(
            state
                .recipe_service
                .create_recipe(domain::Recipe::from_create(recipe_request, auth_user.user))
                .await?
                .into(),
        ),
    ))
}

pub async fn update_recipe(
    State(state): State<Arc<AppState>>,
    ExtractAuthUser(auth_user): ExtractAuthUser,
    Json(recipe_request): Json<UpdateRecipe>,
) -> anyhow::Result<Json<GetRecipe>, AppError> {
    let recipe = state
        .recipe_service
        .get_recipe_by_id(recipe_request.id)
        .await?;

    if recipe.author != auth_user.user {
        return Err(AppError::Unauthorized(String::from(format!(
            "Unable to update recipe belonging to another user {}",
            recipe.author.name
        ))));
    }

    Ok(Json(
        state
            .recipe_service
            .update_recipe(domain::Recipe::from_update(recipe_request, auth_user.user))
            .await?
            .into(),
    ))
}

pub async fn delete_recipe(
    State(state): State<Arc<AppState>>,
    ExtractAuthUser(auth_user): ExtractAuthUser,
    Path(id): Path<i32>,
) -> anyhow::Result<Json<GetRecipe>, AppError> {
    let recipe = state.recipe_service.get_recipe_by_id(id).await?;

    if recipe.author != auth_user.user {
        return Err(AppError::Unauthorized(String::from(format!(
            "Unable to delete recipe belonging to another user {}",
            recipe.author.name
        ))));
    }

    Ok(Json(
        state.recipe_service.delete_recipe_by_id(id).await?.into(),
    ))
}
