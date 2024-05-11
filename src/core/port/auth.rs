use crate::core::domain;
use async_trait::async_trait;

#[cfg(test)]
use mockall::{automock, predicate::*};

#[cfg_attr(test, automock)]
#[async_trait]
pub trait AuthUserRepository {
    async fn get_auth_user_credentials(
        &self,
        username: String,
    ) -> Result<domain::AuthUserCredentials, domain::auth::Error>;
    async fn create_auth_user_credentials(
        &self,
        auth_user: domain::AuthUserCredentials,
    ) -> Result<domain::AuthUserCredentials, domain::auth::Error>;
}

#[async_trait]
pub trait AuthUserService<C, V> {
    async fn validate(&self, credentials: V) -> Result<domain::AuthUser, domain::auth::Error>;
    async fn create_auth_user(
        &self,
        user: domain::User,
        credentials: C,
    ) -> Result<domain::AuthUser, domain::auth::Error>;
}
