use std::error::Error;

use::syringe::container::Container;
use syringe::test::user_repo::{UserMockRepo, UserRepo};

fn main() -> Result<(),Box<dyn Error>> {
    let mut container: Container = Container::new();
    container.register::<UserMockRepo>();
    let user_repo = container.get::<dyn UserRepo>();
    let user = user_repo.find_one("".to_string());
    println!("{:?}",user);
    Ok(())
}
