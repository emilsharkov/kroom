use std::collections::HashMap;
use std::any::{Any, TypeId, type_name};
use std::sync::Arc;
use crate::injectable::Injectable;

type Constructor = fn(&Container) -> Box<dyn Any>;

pub struct Container {
    interface_constructor_registry: HashMap<TypeId, Constructor>,
}

impl Container {
    pub fn new() -> Self {
        Self {
            interface_constructor_registry: HashMap::new(),
        }
    }

    pub fn get<Interface: ?Sized + 'static>(&self) -> Arc<Interface> {
        let interface_id: TypeId = TypeId::of::<Interface>();
        let interface_constructor: &Constructor = self
            .interface_constructor_registry
            .get(&interface_id)
            .unwrap_or_else(||{
                let interace_name: &str = type_name::<Interface>();
                panic!("Interface {:?} with TypeId {:?} not registered to Container",interace_name,interface_id)
            });

        let any_box: Box<dyn Any> = interface_constructor(self);
        let boxed_arc: Box<Arc<Interface>> = any_box
            .downcast::<Arc<Interface>>()
            .expect("Type mismatch during downcast");
        let arc_trait: Arc<Interface> = *boxed_arc;
        arc_trait
    }

    pub fn register<Interface,Implementation>(&mut self)
    where 
        Interface: ?Sized + 'static,
        Implementation: Injectable<Interface> + 'static,
    {
        let interface_type: TypeId = TypeId::of::<Interface>();
        let erased_constructor: Constructor = |container: &Container| -> Box<dyn Any> {
            let target: Arc<Interface> = Implementation::__syringe_construct(container);
            Box::new(target)
        };
        self.interface_constructor_registry.insert(interface_type, erased_constructor);
    }
}