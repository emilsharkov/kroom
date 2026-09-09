use std::sync::Arc;
use syringe::{container::Container, injectable::Injectable};
use crate::common::user::User;

pub trait UserRepo: Send + Sync {
    fn find_one(&self, user_id: &str) -> User;
}

pub struct UserMockRepo;

impl UserRepo for UserMockRepo {
    fn find_one(&self, _user_id: &str) -> User {
        User {
            id: "mock-id".to_string(),
            name: "mock".to_string(),
        }
    }
}

impl Injectable<dyn UserRepo> for UserMockRepo {
    fn __syringe_construct(_container: &Container) -> Arc<dyn UserRepo> {
        Arc::new(UserMockRepo)
    }
}

impl Injectable<UserMockRepo> for UserMockRepo {
    fn __syringe_construct(_container: &Container) -> Arc<UserMockRepo> {
        Arc::new(UserMockRepo)
    }
}

pub struct UserPgRepo;

impl UserRepo for UserPgRepo {
    fn find_one(&self, _user_id: &str) -> User {
        User {
            id: "pg-id".to_string(),
            name: "pg".to_string(),
        }
    }
}

impl Injectable<dyn UserRepo> for UserPgRepo {
    fn __syringe_construct(_container: &Container) -> Arc<dyn UserRepo> {
        Arc::new(UserPgRepo)
    }
}

impl Injectable<UserPgRepo> for UserPgRepo {
    fn __syringe_construct(_container: &Container) -> Arc<UserPgRepo> {
        Arc::new(UserPgRepo)
    }
}
