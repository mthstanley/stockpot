use crate::core::port;

pub struct UserService {
    user_repository: Box<dyn port::UserRepository>,
}

impl port::UserService for UserService {
    fn get_user(&self, id: u64) -> anyhow::Result<crate::core::domain::User> {
        self.user_repository.get_user_by_id(id)
    }
}