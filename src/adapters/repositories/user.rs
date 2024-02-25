use crate::core::{
    domain::{self},
    port,
};
use async_trait::async_trait;
use log::error;

pub struct PostgresUserRepository {
    db_pool: sqlx::postgres::PgPool,
}

impl PostgresUserRepository {
    pub fn new(db_pool: sqlx::postgres::PgPool) -> PostgresUserRepository {
        PostgresUserRepository { db_pool }
    }
}

#[async_trait]
impl port::UserRepository for PostgresUserRepository {
    async fn get_user_by_id(&self, id: i32) -> Result<domain::User, domain::user::Error> {
        sqlx::query_as("select * from app_user where id = $1")
            .bind(id)
            .fetch_one(&self.db_pool)
            .await
            .map_err(|e| match e {
                sqlx::Error::RowNotFound => domain::user::Error::UserNotFound(id).into(),
                _ => {
                    error!("Unable to get user `{}` due to error: {}", id, e);
                    domain::user::Error::Unexpected.into()
                }
            })
    }
}
