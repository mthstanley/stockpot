use std::fmt::Display;

use serde::Serialize;
use sqlx::FromRow;
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum Error {
    #[error("user with id `{0}` not found")]
    UserNotFound(i32),
    #[error("unexpected error occurred")]
    Unexpected,
}

#[derive(Serialize, Clone, Debug, FromRow, PartialEq)]
pub struct User {
    pub id: Option<i32>,
    pub name: String,
}

impl Display for User {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.id {
            Some(id) => {
                write!(f, "id: {}, ", id)?;
            }
            None => {
                write!(f, "id: None, ")?;
            }
        }
        write!(f, "name: {}", self.name)
    }
}
