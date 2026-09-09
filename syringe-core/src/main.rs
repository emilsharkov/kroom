#[path = "../tests/common/mod.rs"]
mod common;

use std::{error::Error, sync::Arc};
use syringe_core::container::Container;
use common::user_repo::UserRepo;

fn main() -> Result<(), Box<dyn Error>> {
    let mut multi_container = Container::new();
    multi_container.auto_register();

    let user_repos: Vec<Arc<dyn UserRepo>> = multi_container.get_all::<dyn UserRepo>();
    for repo in &user_repos {
        let user = repo.find_one("");
        println!("{:?}", user);
    }

    Ok(())
}
