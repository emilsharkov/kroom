use std::sync::Arc;

use crate::{container::Container, injectable::Injectable, test::user::User};

pub trait UserRepo {
    fn find_one(&self, user_id: String) -> User;
}

pub struct UserMockRepo {}
impl UserRepo for UserMockRepo {
    fn find_one(&self, _user_id: String) -> User {
        User {
            id: "mock-id".to_string(),
            name: "mock".to_string()
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

pub struct UserPgRepo {}
impl UserRepo for UserPgRepo {
    fn find_one(&self, _user_id: String) -> User {
        User {
            id: "pg-id".to_string(),
            name: "pg".to_string()
        }
    }
}

impl Injectable<dyn UserRepo> for UserPgRepo {
    fn __syringe_construct(
        _container: &Container
    ) -> Arc<dyn UserRepo> {
        let user_pg_repo: UserPgRepo = Self {};
        Arc::new(user_pg_repo)
    }
}

impl Injectable<UserPgRepo> for UserPgRepo {
    fn __syringe_construct(
        _container: &Container
    ) -> Arc<UserPgRepo> {
        let user_pg_repo: UserPgRepo = Self {};
        Arc::new(user_pg_repo)
    }
}