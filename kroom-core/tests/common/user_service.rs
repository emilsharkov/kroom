use std::{any::{TypeId, type_name}, sync::Arc};
use kroom_core::{container::Container, injectable::Injectable};
use crate::common::{user::User, user_repo::UserRepo};

pub trait UserService {
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
    fn __kroom_construct(container: &Container) -> Arc<dyn UserService> {
        let repo = container.get::<dyn UserRepo>();
        Arc::new(DefaultUserService { repo })
    }

    fn __kroom_scope() -> kroom_core::scope::Scope {
        kroom_core::scope::Scope::Singleton
    }

    fn __kroom_dependent_types() -> Vec<(std::any::TypeId,String)> {
        vec![
            (
                TypeId::of::<dyn UserRepo>(),
                type_name::<dyn UserRepo>().to_string()
            )
        ]
    }

    fn __kroom_interface_name() -> String {
        std::any::type_name::<dyn UserService>().to_string()
    }

    fn __kroom_implementation_name() -> String {
        std::any::type_name::<DefaultUserService>().to_string()
    }
}

impl Injectable<DefaultUserService> for DefaultUserService {
    fn __kroom_construct(container: &Container) -> Arc<DefaultUserService> {
        let repo = container.get::<dyn UserRepo>();
        Arc::new(DefaultUserService { repo })
    }

    fn __kroom_scope() -> kroom_core::scope::Scope {
        kroom_core::scope::Scope::Singleton
    }

    fn __kroom_dependent_types() -> Vec<(std::any::TypeId,String)> {
        vec![
            (
                TypeId::of::<dyn UserRepo>(),
                type_name::<dyn UserRepo>().to_string()
            )
        ]
    }

    fn __kroom_interface_name() -> String {
        std::any::type_name::<DefaultUserService>().to_string()
    }

    fn __kroom_implementation_name() -> String {
        std::any::type_name::<DefaultUserService>().to_string()
    }
}
