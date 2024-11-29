use std::fmt::Display;

use super::{user, User};
use secrecy::Secret;
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum Error {
    #[error("user with username `{0}` not found")]
    AuthUserNotFound(String),
    #[error("user with id `{0}` not found")]
    UserNotFound(i32),
    #[error("Invalid credentials")]
    InvalidAuth,
    #[error("unexpected error occurred")]
    Unexpected,
}

impl From<user::Error> for Error {
    fn from(value: user::Error) -> Self {
        match value {
            user::Error::UserNotFound(id) => Self::UserNotFound(id),
            user::Error::Unexpected => Self::Unexpected,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub aud: String,
    pub sub: String,
    pub exp: u64,
}

#[derive(Serialize, Clone, Debug)]
pub struct UsernameAndPassword {
    pub username: String,
    #[serde(skip_serializing)]
    pub password: Secret<String>,
}

#[derive(Serialize, Clone, Debug)]
pub enum UserCredentials {
    UsernameAndPassword(UsernameAndPassword),
    JwtToken(String),
}

#[derive(Serialize, Clone, Debug)]
pub struct AuthUserCredentials {
    pub id: Option<i32>,
    pub username: String,
    #[serde(skip_serializing)]
    pub password_hash: Secret<String>,
    pub user_id: i32,
}

impl Display for AuthUserCredentials {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.id {
            Some(id) => {
                write!(f, "id: {}, ", id)?;
            }
            None => {
                write!(f, "id: None, ")?;
            }
        }
        write!(f, "username: {}, ", self.username)?;
        write!(f, "user_id: {}", self.user_id)
    }
}

#[derive(Serialize)]
pub struct AuthUser {
    pub username: String,
    pub user: User,
}
