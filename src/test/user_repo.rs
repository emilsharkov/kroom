use std::sync::Arc;

use crate::{container::Container, injectable::Injectable, test::user::User};

pub trait UserRepo {
    fn find_one(&self, user_id: String) -> User;
}

pub struct UserMockRepo {}
impl UserRepo for UserMockRepo {
    fn find_one(&self, _user_id: String) -> User {
        User {
            id: "123-456".to_string(),
            name: "Emil".to_string()
        }
    }
}

impl Injectable<dyn UserRepo> for UserMockRepo {
    fn __syringe_construct(
        _container: &Container
    ) -> Arc<dyn UserRepo> {
        let user_mock_repo: UserMockRepo = Self {};
        Arc::new(user_mock_repo)
    }
}

impl Injectable<UserMockRepo> for UserMockRepo {
    fn __syringe_construct(
        _container: &Container
    ) -> Arc<UserMockRepo> {
        let user_mock_repo: UserMockRepo = Self {};
        Arc::new(user_mock_repo)
    }
}