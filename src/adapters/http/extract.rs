use std::sync::Arc;

use axum::{
    async_trait,
    extract::{FromRef, FromRequestParts},
    http::{request::Parts, HeaderMap},
};
use headers::{
    self,
    authorization::{Basic, Bearer},
    Authorization, HeaderMapExt,
};
use secrecy::Secret;
use serde::de::DeserializeOwned;

use crate::core::domain;

use super::{error::AppError, AppState};
use log::error;

pub struct Path<T>(pub T);

#[async_trait]
impl<S, T> FromRequestParts<S> for Path<T>
where
    T: DeserializeOwned + Send,
    S: Send + Sync,
{
    type Rejection = AppError;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        match axum::extract::Path::<T>::from_request_parts(parts, state).await {
            Ok(value) => Ok(Self(value.0)),
            Err(rejection) => Err(AppError::PathParseError(rejection)),
        }
    }
}

pub struct ExtractAuthUser(pub domain::AuthUser);

#[async_trait]
impl<S> FromRequestParts<S> for ExtractAuthUser
where
    Arc<AppState>: FromRef<S>,
    S: Send + Sync,
{
    type Rejection = AppError;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let header_map = HeaderMap::from_request_parts(parts, state)
            .await
            .map_err(|e| {
                error!("Failed to authenticate user due to error: {}", e);
                domain::auth::Error::InvalidAuth
            })?;

        let credentials;
        if let Some(basic_auth) = header_map.typed_get::<Authorization<Basic>>() {
            credentials = Ok(domain::UserCredentials::UsernameAndPassword(
                domain::auth::UsernameAndPassword {
                    username: basic_auth.username().to_owned(),
                    password: Secret::from(basic_auth.password().to_owned()),
                },
            ));
        } else if let Some(bearer_auth) = header_map.typed_get::<Authorization<Bearer>>() {
            credentials = Ok(domain::UserCredentials::JwtToken(
                bearer_auth.token().to_owned(),
            ));
        } else {
            credentials = Err(domain::auth::Error::InvalidAuth);
        }

        let app_state = Arc::<AppState>::from_ref(state);
        Ok(ExtractAuthUser(
            app_state.auth_user_service.validate(credentials?).await?,
        ))
    }
}
