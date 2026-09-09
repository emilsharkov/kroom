#[path = "../tests/common/mod.rs"]
mod common;

use std::{error::Error, sync::Arc};
use syringe::container::Container;
use common::{
    user_repo::{UserMockRepo, UserPgRepo, UserRepo},
    user_service::{DefaultUserService, UserService},
};

fn main() -> Result<(), Box<dyn Error>> {
    let mut multi_container = Container::new();
    multi_container.register::<dyn UserRepo, UserMockRepo>();
    multi_container.register::<dyn UserRepo, UserPgRepo>();

    let user_repos: Vec<Arc<dyn UserRepo>> = multi_container.get_all::<dyn UserRepo>();
    for repo in &user_repos {
        let user = repo.find_one("");
        println!("{:?}", user);
    }

    let mut container = Container::new();
    container.register::<UserMockRepo, UserMockRepo>();
    container.register::<UserPgRepo, UserPgRepo>();

    let mock_user_repo = container.get::<UserMockRepo>();
    let mock_user = mock_user_repo.find_one("");
    println!("{:?}", mock_user);

    let pg_user_repo = container.get::<UserPgRepo>();
    let pg_user = pg_user_repo.find_one("");
    println!("{:?}", pg_user);

    container.register::<dyn UserRepo, UserMockRepo>();
    container.register::<dyn UserService, DefaultUserService>();

    let user_service = container.get::<dyn UserService>();
    let service_user = user_service.get_user("");
    println!("{:?}", service_user);

    Ok(())
}
