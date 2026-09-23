use std::sync::Arc;
use kroom_core::{container::Container, injectable::Injectable, registration::Registration};
use crate::common::user::User;

pub trait UserRepo {
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
    fn __kroom_construct(_container: &Container) -> Arc<dyn UserRepo> {
        Arc::new(UserMockRepo)
    }

    fn __kroom_scope() -> kroom_core::scope::Scope {
        kroom_core::scope::Scope::Singleton
    }

    fn __kroom_dependent_types() -> Vec<(std::any::TypeId,String)> {
        vec![]
    }

    fn __kroom_interface_name() -> String {
        std::any::type_name::<dyn UserRepo>().to_string()
    }
    
    fn __kroom_implementation_name() -> String {
        std::any::type_name::<UserMockRepo>().to_string()
    }
}

inventory::submit! {
    Registration::of::<dyn UserRepo,UserMockRepo>()
}

impl Injectable<UserMockRepo> for UserMockRepo {
    fn __kroom_construct(_container: &Container) -> Arc<UserMockRepo> {
        Arc::new(UserMockRepo)
    }

    fn __kroom_scope() -> kroom_core::scope::Scope {
        kroom_core::scope::Scope::Singleton
    }

    fn __kroom_dependent_types() -> Vec<(std::any::TypeId,String)> {
        vec![]
    }

    fn __kroom_interface_name() -> String {
        std::any::type_name::<UserMockRepo>().to_string()
    }

    fn __kroom_implementation_name() -> String {
        std::any::type_name::<UserMockRepo>().to_string()
    }
}

inventory::submit! {
    Registration::of::<UserMockRepo,UserMockRepo>()
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
    fn __kroom_construct(_container: &Container) -> Arc<dyn UserRepo> {
        Arc::new(UserPgRepo {})
    }

    fn __kroom_scope() -> kroom_core::scope::Scope {
        kroom_core::scope::Scope::Singleton
    }

    fn __kroom_dependent_types() -> Vec<(std::any::TypeId,String)> {
        vec![]
    }

    fn __kroom_interface_name() -> String {
        std::any::type_name::<dyn UserRepo>().to_string()
    }

    fn __kroom_implementation_name() -> String {
        std::any::type_name::<UserPgRepo>().to_string()
    }
}

inventory::submit! {
    Registration::of::<dyn UserRepo,UserPgRepo>()
}

impl Injectable<UserPgRepo> for UserPgRepo {
    fn __kroom_construct(_container: &Container) -> Arc<UserPgRepo> {
        Arc::new(UserPgRepo {})
    }

    fn __kroom_scope() -> kroom_core::scope::Scope {
        kroom_core::scope::Scope::Singleton
    }

    fn __kroom_dependent_types() -> Vec<(std::any::TypeId,String)> {
        vec![]
    }

    fn __kroom_interface_name() -> String {
        std::any::type_name::<UserPgRepo>().to_string()
    }

    fn __kroom_implementation_name() -> String {
        std::any::type_name::<UserPgRepo>().to_string()
    }
}

inventory::submit! {
    Registration::of::<UserPgRepo,UserPgRepo>()
}