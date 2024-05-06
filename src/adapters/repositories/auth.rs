use crate::core::{
    domain::{self, auth::AuthUserCredentials},
    port,
};
use async_trait::async_trait;
use log::error;
use secrecy::{ExposeSecret, Secret};
use sqlx::{postgres::PgRow, FromRow, Row};

pub struct PostgresAuthUserRepository {
    db_pool: sqlx::postgres::PgPool,
}

impl PostgresAuthUserRepository {
    pub fn new(db_pool: sqlx::postgres::PgPool) -> PostgresAuthUserRepository {
        PostgresAuthUserRepository { db_pool }
    }
}

impl FromRow<'_, PgRow> for AuthUserCredentials {
    fn from_row(row: &'_ PgRow) -> Result<Self, sqlx::Error> {
        Ok(Self {
            id: row.try_get("id")?,
            username: row.try_get("username")?,
            password_hash: Secret::new(row.try_get("password_hash")?),
            user_id: row.try_get("app_user")?,
        })
    }
}

#[async_trait]
impl port::AuthUserRepository for PostgresAuthUserRepository {
    async fn get_auth_user_credentials(
        &self,
        username: String,
    ) -> Result<domain::AuthUserCredentials, domain::auth::Error> {
        sqlx::query_as("select * from auth_user where username = $1")
            .bind(&username)
            .fetch_one(&self.db_pool)
            .await
            .map_err(|e| match e {
                sqlx::Error::RowNotFound => domain::auth::Error::AuthUserNotFound(username).into(),
                _ => {
                    error!("Unable to get auth user `{}` due to error: {}", username, e);
                    domain::auth::Error::Unexpected.into()
                }
            })
    }

    async fn create_auth_user_credentials(
        &self,
        auth_user: domain::AuthUserCredentials,
    ) -> Result<domain::AuthUserCredentials, domain::auth::Error> {
        sqlx::query_as(
            "insert into auth_user (username, password_hash, app_user) values ($1, $2, $3) returning *",
        )
        .bind(&auth_user.username)
        .bind(&auth_user.password_hash.expose_secret())
        .bind(&auth_user.user_id)
        .fetch_one(&self.db_pool)
        .await
        .map_err(|e| match e {
            _ => {
                error!(
                    "Unable to insert auth user `{}` due to error: {}",
                    auth_user, e
                );
                domain::auth::Error::Unexpected.into()
            }
        })
    }
}
