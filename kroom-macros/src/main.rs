use std::error::Error;

use kroom_macros::injectable;
use kroom_core::container::Container;

#[injectable]
struct Water;
impl Water {
    fn slosh(&self) {
        println!("I am water sloshing");
    }
}

trait Granular {
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

trait Trait {
    fn action(&self);
}

impl Trait for Concrete {
    fn action(&self) {
        println!("Action complete")
    }
}

#[injectable(type = dyn Trait)]
struct Concrete {
    #[inject]
    liquid: Water,
    #[multi_inject]
    solids: Vec<dyn Granular>
}

fn main() -> Result<(),Box<dyn Error>> {
    let mut container = Container::new();
    container.auto_register();
    let dyn_trait_struct = container.get::<dyn Trait>();
    let concrete_struct = container.get::<Concrete>();
    let dyn_solid_structs = container.get_all::<dyn Granular>();

    println!("Dyn Struct");
    dyn_trait_struct.action();
    println!("Concrete Struct");
    concrete_struct.action();
    concrete_struct.liquid.slosh();
    concrete_struct.solids.iter().for_each(|solid| solid.get_volume());
    println!("Dyn Solid Structs");
    dyn_solid_structs.iter().for_each(|solid| solid.get_volume());
    Ok(())
}
