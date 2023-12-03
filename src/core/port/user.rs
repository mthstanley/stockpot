use crate::core::domain;

pub trait UserRepository {
    fn get_user_by_id(&self, id: u64) -> anyhow::Result<domain::User>;
}

pub trait UserService {
    fn get_user(&self, id: u64) -> anyhow::Result<domain::User>;
}
