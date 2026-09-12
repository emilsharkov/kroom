use std::sync::Arc;
use kroom_core::{container::Container, injectable::Injectable};
use crate::common::{user::User, user_repo::UserRepo};

pub trait UserService: Send + Sync {
    fn get_user(&self, id: &str) -> User;
}

pub struct DefaultUserService {
    repo: Arc<dyn UserRepo>,
}

impl UserService for DefaultUserService {
    fn get_user(&self, id: &str) -> User {
        self.repo.find_one(id)
    }
}

impl Injectable<dyn UserService> for DefaultUserService {
    fn __syringe_construct(container: &Container) -> Arc<dyn UserService> {
        let repo = container.get::<dyn UserRepo>();
        Arc::new(DefaultUserService { repo })
    }
}

impl Injectable<DefaultUserService> for DefaultUserService {
    fn __syringe_construct(container: &Container) -> Arc<DefaultUserService> {
        let repo = container.get::<dyn UserRepo>();
        Arc::new(DefaultUserService { repo })
    }
}
