use std::error::Error;

use::syringe::container::Container;
use syringe::test::user_repo::{UserMockRepo, UserPgRepo, UserRepo};

fn main() -> Result<(),Box<dyn Error>> {
    let mut container: Container = Container::new();

    container.register::<dyn UserRepo,UserMockRepo>();
    let user_repo = container.get::<dyn UserRepo>();
    let user1 = user_repo.find_one("".to_string());
    println!("{:?}",user1);

    container.register::<UserMockRepo,UserMockRepo>();
    let mock_user_repo = container.get::<UserMockRepo>();
    let user2 = mock_user_repo.find_one("".to_string());
    println!("{:?}",user2);

    container.register::<UserPgRepo,UserPgRepo>();
    let pg_user_repo = container.get::<UserPgRepo>();
    let user3 = pg_user_repo.find_one("".to_string());
    println!("{:?}",user3);

    Ok(())
}
