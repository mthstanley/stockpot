use crate::core::{
    domain::{self},
    port,
};
use async_trait::async_trait;

#[cfg(test)]
use mockall::predicate::*;

pub struct DefaultUserService {
    user_repository: Box<dyn port::UserRepository + Send + Sync>,
}

impl DefaultUserService {
    pub fn new(user_repository: Box<dyn port::UserRepository + Send + Sync>) -> DefaultUserService {
        DefaultUserService { user_repository }
    }
}

#[async_trait]
impl port::UserService for DefaultUserService {
    async fn get_user(&self, id: i32) -> Result<domain::User, domain::user::Error> {
        let user = self.user_repository.get_user_by_id(id).await?;
        Ok(user)
    }

    async fn create_user(&self, user: domain::User) -> Result<domain::User, domain::user::Error> {
        let user = self.user_repository.create_user(user).await?;
        Ok(user)
    }
}

#[cfg(test)]
mod test {

    use super::*;

    use crate::core::port::user::{MockUserRepository, UserService};

    #[tokio::test]
    async fn test_get_user_successfully() {
        let id = 3;
        let user = domain::User {
            id: Some(id),
            name: "test".into(),
        };
        let mut mock = MockUserRepository::new();
        let user_result = user.clone();
        mock.expect_get_user_by_id()
            .with(eq(id))
            .once()
            .returning(move |_| Ok(user_result.clone()));
        let user_service = DefaultUserService::new(Box::new(mock));
        assert_eq!(user_service.get_user(id).await.unwrap(), user);
    }

    #[tokio::test]
    async fn test_get_user_not_found() {
        let id = 3;
        let mut mock = MockUserRepository::new();
        mock.expect_get_user_by_id()
            .with(eq(id))
            .once()
            .returning(|given_id| Err(domain::user::Error::UserNotFound(given_id)));
        let user_service = DefaultUserService::new(Box::new(mock));
        assert_eq!(
            user_service.get_user(id).await.unwrap_err(),
            domain::user::Error::UserNotFound(id)
        );
    }

    #[tokio::test]
    async fn test_get_user_unexpected_error() {
        let id = 3;
        let mut mock = MockUserRepository::new();
        mock.expect_get_user_by_id()
            .with(eq(id))
            .once()
            .returning(|_| Err(domain::user::Error::Unexpected));
        let user_service = DefaultUserService::new(Box::new(mock));
        assert_eq!(
            user_service.get_user(id).await.unwrap_err(),
            domain::user::Error::Unexpected
        );
    }

    #[tokio::test]
    async fn test_create_user_successfully() {
        let id = Some(3);
        let mut mock = MockUserRepository::new();
        mock.expect_create_user()
            .with(eq(domain::User {
                id: None,
                name: "foo".to_owned(),
            }))
            .once()
            .returning(move |_| {
                Ok(domain::User {
                    id,
                    name: "foo".to_owned(),
                })
            });
        let user_service = DefaultUserService::new(Box::new(mock));
        assert_eq!(
            user_service
                .create_user(domain::User {
                    id: None,
                    name: "foo".to_owned()
                })
                .await
                .unwrap(),
            domain::User {
                id,
                name: "foo".to_owned()
            }
        );
    }
}
