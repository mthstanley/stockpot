use std::sync::Arc;

use axum::{extract::State, routing::get, Json, Router};

use crate::core::domain;

use super::{error::AppError, extract::Path, AppState};

pub fn build_routes() -> Router<Arc<AppState>> {
    Router::new().route("/user/:id", get(user))
}

pub async fn user(
    State(state): State<Arc<AppState>>,
    Path(id): Path<i32>,
) -> anyhow::Result<Json<domain::User>, AppError> {
    let user = state.user_service.get_user(id).await?;
    Ok(Json(user))
}
