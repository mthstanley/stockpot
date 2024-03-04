use serde::Serialize;
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum Error {
    #[error("user with id `{0}` not found")]
    UserNotFound(i32),
    #[error("unexpected error occurred")]
    Unexpected,
}

#[derive(sqlx::FromRow, Serialize, Clone, Debug, PartialEq)]
pub struct User {
    pub id: i32,
    pub name: String,
}
