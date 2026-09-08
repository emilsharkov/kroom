use std::collections::HashMap;
use std::any::{Any, TypeId};
use crate::injectable::Injectable;

type Constructor = fn(&Container) -> Box<dyn Any>;

pub struct Container {
     trait_to_constructor: HashMap<TypeId,Constructor>
}

impl Container {
    pub fn new() -> Self {
        Self {
            trait_to_constructor: HashMap::new()
        }
    }

    pub fn get<T: ?Sized + 'static>(&self) -> Box<T> {
            let id = TypeId::of::<T>();
            let constructor = self
                .trait_to_constructor
                .get(&id)
                .expect("Not implemented");
    
            let boxed_any = constructor(self);
            let boxed_t = boxed_any
                .downcast::<Box<T>>()
                .expect("Type mismatch during downcast");
            *boxed_t
    }

    pub fn register<V: Injectable + 'static>(&mut self) {
        let id = TypeId::of::<V::Target>();
        let erased_constructor: Constructor = |container: &Container| -> Box<dyn Any> {
            let boxed_target: Box<V::Target> = V::__syringe_construct(container);
            Box::new(boxed_target) // Box<Box<V::Target>>, coerced to Box<dyn Any>
        };
        self.trait_to_constructor.insert(id, erased_constructor);
    }
}