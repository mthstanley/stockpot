use log::error;
use std::sync::Arc;

use crate::core::{domain, port};
use argon2::{
    password_hash::{Salt, SaltString},
    Algorithm, Argon2, Params, PasswordHash, PasswordHasher, PasswordVerifier,
};
use async_trait::async_trait;
use secrecy::{ExposeSecret, Secret};
pub struct DefaultAuthUserService {
    auth_user_repository: Box<dyn port::AuthUserRepository + Send + Sync>,
    user_service: Arc<dyn port::UserService + Send + Sync>,
}

impl DefaultAuthUserService {
    pub fn new(
        auth_user_repository: Box<dyn port::AuthUserRepository + Send + Sync>,
        user_service: Arc<dyn port::UserService + Send + Sync>,
    ) -> DefaultAuthUserService {
        DefaultAuthUserService {
            auth_user_repository,
            user_service,
        }
    }
}

#[async_trait]
impl port::AuthUserService<domain::UserCredentials, domain::UserCredentials>
    for DefaultAuthUserService
{
    async fn validate(
        &self,
        credentials: domain::UserCredentials,
    ) -> Result<domain::AuthUser, domain::auth::Error> {
        let user_result;
        let mut expected_password_hash = Secret::new(
            "$argon2id$v=19$m=15000,t=2,p=1$\
        gZiV/M1gPc22ElAH/Jh1Hw$\
        CWOrkoo7oJBQ/iyh7uJ0LO2aLEfrHwTWllSAxT0zRno"
                .to_string(),
        );
        let stored_auth_user_result = self
            .auth_user_repository
            .get_auth_user_credentials(credentials.username.clone())
            .await;
        match stored_auth_user_result {
            Ok(stored_auth_user) => {
                expected_password_hash = stored_auth_user.password_hash;
                user_result = self
                    .user_service
                    .get_user(stored_auth_user.user_id)
                    .await
                    .map_err(|e| e.into());
            }
            Err(e) => {
                user_result = Err(e);
            }
        }

        // we should always do some hash comparison to avoid timing attacks
        Argon2::default()
            .verify_password(
                credentials.password.expose_secret().as_bytes(),
                &PasswordHash::new(expected_password_hash.expose_secret()).map_err(|e| {
                    error!(
                        "Unable to parse stored password hash for user `{}` due to error: {}",
                        credentials.username, e
                    );
                    return domain::auth::Error::Unexpected;
                })?,
            )
            .map_err(|e| {
                error!(
                    "Unable to verify password for user `{}` due to error: {}",
                    credentials.username, e
                );
                return domain::auth::Error::InvalidAuth;
            })?;

        match user_result {
            Ok(user) => Ok(domain::AuthUser { user }),
            Err(error) => match error {
                // if we can't find the auth user or associated user then the
                // credentials are invalid
                domain::auth::Error::AuthUserNotFound(_) | domain::auth::Error::UserNotFound(_) => {
                    Err(domain::auth::Error::InvalidAuth)
                }
                _ => Err(error),
            },
        }
    }

    async fn create_auth_user(
        &self,
        user: domain::User,
        credentials: domain::UserCredentials,
    ) -> Result<domain::AuthUser, domain::auth::Error> {
        match user.id {
            Some(user_id) => {
                self.user_service.get_user(user_id).await?;
                let salt = SaltString::generate(&mut rand::thread_rng());
                let password_hasher = Argon2::new(
                    Algorithm::Argon2id,
                    argon2::Version::V0x13,
                    Params::new(15000, 2, 1, None).map_err(|e| {
                        error!("Unable to create argon2 hasher due to error: {}", e);
                        return domain::auth::Error::Unexpected;
                    })?,
                );
                let password_hash = password_hasher
                    .hash_password(
                        credentials.password.expose_secret().as_bytes(),
                        Salt::from(&salt),
                    )
                    .map_err(|e| {
                        error!("Unable to hash password due to error: {}", e);
                        return domain::auth::Error::Unexpected;
                    })?;
                self.auth_user_repository
                    .create_auth_user_credentials(domain::AuthUserCredentials {
                        id: None,
                        username: credentials.username,
                        user_id,
                        password_hash: Secret::new(password_hash.to_string()),
                    })
                    .await?;
                Ok(domain::AuthUser { user })
            }
            None => Err(domain::auth::Error::Unexpected),
        }
    }
}
