use std::{error::Error, sync::Arc};

use::syringe::container::Container;
use syringe::test::{user::User, user_repo::{UserMockRepo, UserPgRepo, UserRepo}};

fn main() -> Result<(),Box<dyn Error>> {
    let mut container: Container = Container::new();

    container.register::<dyn UserRepo,UserMockRepo>();
    container.register::<dyn UserRepo,UserPgRepo>();
    let user_repos: Vec<Arc<dyn UserRepo>> = container.get_all::<dyn UserRepo>();
    user_repos
        .iter()
        .for_each(|user_repo: &Arc<dyn UserRepo>| {
            let user: User = user_repo.find_one("".to_string());
            println!("{:?}",user)
        });

    container.register::<UserMockRepo,UserMockRepo>();
    let mock_user_repo = container.get::<UserMockRepo>();
    let mock_user = mock_user_repo.find_one("".to_string());
    println!("{:?}",mock_user);

    container.register::<UserPgRepo,UserPgRepo>();
    let pg_user_repo = container.get::<UserPgRepo>();
    let pg_user = pg_user_repo.find_one("".to_string());
    println!("{:?}",pg_user);

    Ok(())
}
