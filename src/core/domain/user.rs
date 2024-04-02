use core::fmt;

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
    pub id: Option<i32>,
    pub name: String,
}

impl fmt::Display for User {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.id {
            Some(id) => write!(f, "id: {} ", id),
            None => write!(f, "id: None "),
        }?;
        write!(f, "name: {}", self.name)
    }
}
