use crate::core::domain;
use async_trait::async_trait;

#[cfg(test)]
use mockall::{automock, predicate::*};

#[cfg_attr(test, automock)]
#[async_trait]
pub trait UserRepository {
    async fn get_user_by_id(&self, id: i32) -> Result<domain::User, domain::user::Error>;
}

#[async_trait]
pub trait UserService {
    async fn get_user(&self, id: i32) -> Result<domain::User, domain::user::Error>;
}
