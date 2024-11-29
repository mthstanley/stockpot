use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
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
    jwt_token_secret: String,
    jwt_token_audience: String,
    jwt_token_expiration: Duration,
}

impl DefaultAuthUserService {
    pub fn new(
        auth_user_repository: Box<dyn port::AuthUserRepository + Send + Sync>,
        user_service: Arc<dyn port::UserService + Send + Sync>,
        jwt_token_secret: String,
    ) -> DefaultAuthUserService {
        DefaultAuthUserService {
            auth_user_repository,
            user_service,
            jwt_token_secret,
            jwt_token_audience: "https://api.stockpot.com".to_owned(),
            jwt_token_expiration: chrono::Duration::days(7),
        }
    }
}

#[async_trait]
impl port::AuthUserService for DefaultAuthUserService {
    async fn validate(
        &self,
        credentials: domain::UserCredentials,
    ) -> Result<domain::AuthUser, domain::auth::Error> {
        match credentials {
            domain::UserCredentials::UsernameAndPassword(username_and_password) => {
                let user_result;
                let mut expected_password_hash = Secret::new(
                    "$argon2id$v=19$m=15000,t=2,p=1$\
        gZiV/M1gPc22ElAH/Jh1Hw$\
        CWOrkoo7oJBQ/iyh7uJ0LO2aLEfrHwTWllSAxT0zRno"
                        .to_string(),
                );
                let stored_auth_user_result = self
                    .auth_user_repository
                    .get_auth_user_credentials(username_and_password.username.clone())
                    .await;
                match stored_auth_user_result {
                    Ok(stored_auth_user) => {
                        expected_password_hash = stored_auth_user.password_hash.clone();
                        user_result = self
                            .user_service
                            .get_user(stored_auth_user.user_id)
                            .await
                            .map_err(|e| e.into())
                            .map(|user| (stored_auth_user.username, user));
                    }
                    Err(e) => {
                        user_result = Err(e);
                    }
                }

                // we should always do some hash comparison to avoid timing attacks
                Argon2::default()
                    .verify_password(
                        username_and_password.password.expose_secret().as_bytes(),
                        &PasswordHash::new(expected_password_hash.expose_secret()).map_err(
                            |e| {
                                error!(
                        "Unable to parse stored password hash for user `{}` due to error: {}",
                        username_and_password.username, e
                    );
                                return domain::auth::Error::Unexpected;
                            },
                        )?,
                    )
                    .map_err(|e| {
                        error!(
                            "Unable to verify password for user `{}` due to error: {}",
                            username_and_password.username, e
                        );
                        return domain::auth::Error::InvalidAuth;
                    })?;

                match user_result {
                    Ok(user) => Ok(domain::AuthUser {
                        username: user.0,
                        user: user.1,
                    }),
                    Err(error) => match error {
                        // if we can't find the auth user or associated user then the
                        // credentials are invalid
                        domain::auth::Error::AuthUserNotFound(_)
                        | domain::auth::Error::UserNotFound(_) => {
                            Err(domain::auth::Error::InvalidAuth)
                        }
                        _ => Err(error),
                    },
                }
            }
            domain::UserCredentials::JwtToken(token) => {
                let mut validation = Validation::new(jsonwebtoken::Algorithm::default());
                validation.set_audience(&[self.jwt_token_audience.clone()]);
                validation.set_required_spec_claims(&["aud", "sub", "exp"]);
                let token_data = decode::<domain::auth::Claims>(
                    &token,
                    &DecodingKey::from_secret(self.jwt_token_secret.as_bytes()),
                    &validation,
                )
                .map_err(|e| {
                    error!("Unable to decode token due to error: {}", e);
                    domain::auth::Error::InvalidAuth
                })?;

                let auth_user = self
                    .auth_user_repository
                    .get_auth_user_credentials(token_data.claims.sub)
                    .await?;

                let user = self.user_service.get_user(auth_user.user_id).await?;

                Ok(domain::AuthUser {
                    username: auth_user.username,
                    user,
                })
            }
        }
    }

    async fn create_auth_user(
        &self,
        user: domain::User,
        credentials: domain::auth::UsernameAndPassword,
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
                        username: credentials.username.clone(),
                        user_id,
                        password_hash: Secret::new(password_hash.to_string()),
                    })
                    .await?;
                Ok(domain::AuthUser {
                    username: credentials.username,
                    user,
                })
            }
            None => Err(domain::auth::Error::Unexpected),
        }
    }

    fn generate_jwt_token(
        &self,
        auth_user: domain::AuthUser,
    ) -> Result<String, domain::auth::Error> {
        return encode(
            &Header::default(),
            &domain::auth::Claims {
                aud: self.jwt_token_audience.clone(),
                sub: auth_user.username.clone(),
                exp: (Utc::now() + self.jwt_token_expiration).timestamp() as u64,
            },
            &EncodingKey::from_secret(self.jwt_token_secret.as_bytes()),
        )
        .map_err(|e| {
            error!(
                "Unable to create JWT token for user {} due to error: {}",
                auth_user.username, e
            );
            domain::auth::Error::Unexpected
        });
    }
}
