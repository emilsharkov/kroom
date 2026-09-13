use std::sync::Arc;

use kroom_macros::injectable;
use kroom_core::container::Container;

trait Trait {}

impl Trait for Concrete {}

#[injectable(type = dyn Trait)]
struct Concrete {
    #[inject]
    is_true: Arc<bool>,
}

fn main() {
    let mut container = Container::new();
    container.auto_register();
    let dyn_trait_struct = container.get::<dyn Trait>();
    let concrete_struct = container.get::<Concrete>();
    println!("Succeeded");
}
