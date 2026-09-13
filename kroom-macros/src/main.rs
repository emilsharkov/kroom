use kroom_macros::injectable;
use kroom_core::container::Container;

#[injectable]
struct Asphalt;

impl Asphalt {
    fn exist(&self) {
        println!("I exist!")
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
    asphalt: Asphalt,
}

fn main() {
    let mut container = Container::new();
    container.auto_register();
    let dyn_trait_struct = container.get::<dyn Trait>();
    let concrete_struct = container.get::<Concrete>();

    dyn_trait_struct.action();
    concrete_struct.action();
    concrete_struct.asphalt.exist();
}
