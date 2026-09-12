mod common;

use std::sync::Arc;
use kroom_core::container::Container;
use common::{
    user_repo::{UserMockRepo, UserPgRepo, UserRepo},
    user_service::{DefaultUserService, UserService},
};

#[test]
fn test_register_and_get_concrete_types() {
    let mut container = Container::new();

    container.register::<UserMockRepo, UserMockRepo>();
    container.register::<UserPgRepo, UserPgRepo>();

    let mock_repo = container.get::<UserMockRepo>();
    let pg_repo = container.get::<UserPgRepo>();

    let mock_user = mock_repo.find_one("any");
    let pg_user = pg_repo.find_one("any");

    assert_eq!(mock_user.id, "mock-id");
    assert_eq!(mock_user.name, "mock");
    assert_eq!(pg_user.id, "pg-id");
    assert_eq!(pg_user.name, "pg");
}

#[test]
fn test_register_and_get_trait_interface() {
    let mut container = Container::new();

    container.register::<dyn UserRepo, UserMockRepo>();
    let user_repo: Arc<dyn UserRepo> = container.get::<dyn UserRepo>();

    let user = user_repo.find_one("123");
    assert_eq!(user.id, "mock-id");
}

#[test]
fn test_get_all_multiple_implementations() {
    let mut container = Container::new();

    container.register::<dyn UserRepo, UserMockRepo>();
    container.register::<dyn UserRepo, UserPgRepo>();

    let repos: Vec<Arc<dyn UserRepo>> = container.get_all::<dyn UserRepo>();
    assert_eq!(repos.len(), 2);

    let mock_res = repos[0].find_one("1");
    let pg_res = repos[1].find_one("2");

    assert_eq!(mock_res.id, "mock-id");
    assert_eq!(pg_res.id, "pg-id");
}

#[test]
fn test_get_all_single_implementation() {
    let mut container = Container::new();

    container.register::<dyn UserRepo, UserMockRepo>();

    let repos: Vec<Arc<dyn UserRepo>> = container.get_all::<dyn UserRepo>();
    assert_eq!(repos.len(), 1);
    assert_eq!(repos[0].find_one("1").id, "mock-id");
}

#[test]
fn test_nested_dependency_resolution() {
    let mut container = Container::new();

    // Register repo interface -> mock implementation
    container.register::<dyn UserRepo, UserMockRepo>();
    // Register service interface -> default implementation (which requests dyn UserRepo)
    container.register::<dyn UserService, DefaultUserService>();

    let service: Arc<dyn UserService> = container.get::<dyn UserService>();
    let user = service.get_user("test-id");

    assert_eq!(user.id, "mock-id");
    assert_eq!(user.name, "mock");
}

#[test]
#[should_panic(expected = "not registered to Container")]
fn test_get_unregistered_interface_panics() {
    let container = Container::new();
    let _ = container.get::<dyn UserRepo>();
}

#[test]
#[should_panic(expected = "not registered to Container")]
fn test_get_all_unregistered_interface_panics() {
    let container = Container::new();
    let _ = container.get_all::<dyn UserRepo>();
}

#[test]
#[should_panic(expected = "expects one implementation")]
fn test_get_with_multiple_implementations_panics() {
    let mut container = Container::new();

    container.register::<dyn UserRepo, UserMockRepo>();
    container.register::<dyn UserRepo, UserPgRepo>();

    // Container::get() expects exactly one implementation registered
    let _ = container.get::<dyn UserRepo>();
}
