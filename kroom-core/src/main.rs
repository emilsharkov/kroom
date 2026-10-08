#[path = "../tests/common/mod.rs"]
mod common;

use std::{error::Error, sync::Arc};
use common::user_repo::UserRepo;
use kroom_core::{container::Container, container_builder::ContainerBuilder};

fn main() -> Result<(), Box<dyn Error>> {
    let multi_container: Container = ContainerBuilder::build()?;

    let user_repos: Vec<Arc<dyn UserRepo>> = multi_container.get_all::<dyn UserRepo>();
    for repo in &user_repos {
        let user = repo.find_one("");
        println!("{:?}", user);
    }

    Ok(())
}
