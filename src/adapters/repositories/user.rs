use crate::core::{
    domain::{self, User},
    port,
};

pub struct UserRepository {}

impl port::UserRepository for UserRepository {
    fn get_user_by_id(&self, id: u64) -> anyhow::Result<domain::User> {
        Ok(User { id })
    }
}
