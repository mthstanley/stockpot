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
pub trait AuthUserService {
    async fn validate(
        &self,
        credentials: domain::UserCredentials,
    ) -> Result<domain::AuthUser, domain::auth::Error>;
    async fn create_auth_user(
        &self,
        user: domain::User,
        credentials: domain::auth::UsernameAndPassword,
    ) -> Result<domain::AuthUser, domain::auth::Error>;
    fn generate_jwt_token(
        &self,
        auth_user: domain::AuthUser,
    ) -> Result<String, domain::auth::Error>;
}
