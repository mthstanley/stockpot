use std::sync::Arc;

use axum::{
    extract::State,
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};

use crate::core::domain;

use super::{error::AppError, extract::Path, AppState};

#[derive(Serialize)]
pub struct GetUser {
    pub id: i32,
    pub name: String,
}

impl From<domain::User> for GetUser {
    fn from(value: domain::User) -> Self {
        GetUser {
            id: match value.id {
                Some(value_id) => value_id,
                None => -1,
            },
            name: value.name,
        }
    }
}

#[derive(Deserialize)]
pub struct CreateUser {
    pub name: String,
}

impl From<CreateUser> for domain::User {
    fn from(value: CreateUser) -> Self {
        domain::User {
            id: None,
            name: value.name,
        }
    }
}

pub fn build_routes() -> Router<Arc<AppState>> {
    Router::new()
        .route("/user/:id", get(get_user))
        .route("/user", post(create_user))
}

pub async fn get_user(
    State(state): State<Arc<AppState>>,
    Path(id): Path<i32>,
) -> anyhow::Result<Json<GetUser>, AppError> {
    let user = state.user_service.get_user(id).await?;
    Ok(Json(user.into()))
}

pub async fn create_user(
    State(state): State<Arc<AppState>>,
    Json(user_request): Json<CreateUser>,
) -> anyhow::Result<(StatusCode, Json<GetUser>), AppError> {
    let user = state.user_service.create_user(user_request.into()).await?;
    Ok((StatusCode::CREATED, Json(user.into())))
}
