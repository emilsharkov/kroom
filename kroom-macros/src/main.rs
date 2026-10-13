use std::error::Error;

use kroom_core::{scope::Scope, container::Container, container_builder::ContainerBuilder};
use kroom_macros::injectable;

#[injectable]
struct Water;
impl Water {
    fn slosh(&self) {
        println!("I am water sloshing");
    }
}

trait Granular: Send + Sync {
    fn get_volume(&self);
}

#[injectable(type = dyn Granular)]
struct Sand;
impl Granular for Sand {
    fn get_volume(&self) {
        println!("Sand's volume");
    }
}

#[injectable(type = dyn Granular)]
struct Gravel;
impl Granular for Gravel {
    fn get_volume(&self) {
        println!("Gravel's volume");
    }
}

trait Trait: Send + Sync {
    fn action(&self);
}

#[injectable(type = dyn Trait)]
struct Concrete {
    #[inject]
    liquid: Water,
    #[multi_inject]
    solids: Vec<dyn Granular>,
}

impl Trait for Concrete {
    fn action(&self) {
        println!("Action complete")
    }
}

impl Concrete {
    fn display(&self) {
        self.liquid.slosh();        
        self.solids.iter().for_each(|solid| {
            solid.get_volume();
        })
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let container: Container = ContainerBuilder::build()?;
    let dyn_trait_struct = container.get::<dyn Trait>();
    let concrete_struct = container.get::<Concrete>();

    println!("Dyn Struct Action");
    dyn_trait_struct.action();
    println!("Concrete Struct Action");
    concrete_struct.action();
    println!("Concrete Struct Display");
    concrete_struct.display();
    Ok(())
}
