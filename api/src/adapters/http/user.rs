use std::sync::Arc;

use axum::{
    extract::State,
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};

use crate::core::domain;

use super::{
    error::AppError,
    extract::{ExtractAuthUser, Path},
    AppState,
};

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct GetUser {
    pub id: i32,
    pub name: String,
}

#[derive(Deserialize, Clone)]
pub struct CreateUser {
    pub username: String,
    pub password: String,
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct GetToken {
    pub token: String,
}

impl From<CreateUser> for domain::User {
    fn from(value: CreateUser) -> Self {
        Self {
            id: None,
            name: value.name,
        }
    }
}

impl From<domain::User> for GetUser {
    fn from(value: domain::User) -> Self {
        Self {
            id: value.id.unwrap_or(-1),
            name: value.name,
        }
    }
}

impl From<CreateUser> for domain::auth::UsernameAndPassword {
    fn from(value: CreateUser) -> Self {
        Self {
            username: value.username,
            password: secrecy::Secret::new(value.password),
        }
    }
}

pub fn build_routes() -> Router<Arc<AppState>> {
    Router::new()
        .route("/user/:id", get(get_user))
        .route("/user", post(create_user))
        .route("/user/auth", get(get_auth_user))
        .route("/user/token", post(create_token))
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
    let user = state
        .user_service
        .create_user(user_request.clone().into())
        .await?;
    state
        .auth_user_service
        .create_auth_user(user.clone(), user_request.clone().into())
        .await?;
    Ok((StatusCode::CREATED, Json(user.into())))
}

pub async fn create_token(
    State(state): State<Arc<AppState>>,
    ExtractAuthUser(auth_user): ExtractAuthUser,
) -> anyhow::Result<Json<GetToken>, AppError> {
    Ok(Json(GetToken {
        token: state.auth_user_service.generate_jwt_token(auth_user)?,
    }))
}

// TODO: remove
#[derive(Serialize)]
pub struct GetAuthUser {
    pub message: String,
}

pub async fn get_auth_user(
    State(_): State<Arc<AppState>>,
    ExtractAuthUser(auth_user): ExtractAuthUser,
) -> anyhow::Result<Json<GetAuthUser>, AppError> {
    Ok(Json(GetAuthUser {
        message: format!("Successful authentication for {}", auth_user.user.name),
    }))
}
