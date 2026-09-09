use std::collections::HashMap;
use std::any::{Any, TypeId, type_name, type_name_of_val};
use std::sync::Arc;
use crate::injectable::Injectable;

type Constructor = fn(&Container) -> Box<dyn Any>;

pub struct Container {
    interface_constructor_registry: HashMap<TypeId, Vec<Constructor>>,
}

impl Container {
    pub fn new() -> Self {
        Self {
            interface_constructor_registry: HashMap::new(),
        }
    }
    
    pub fn register<Interface,Implementation>(&mut self)
    where 
        Interface: ?Sized + 'static,
        Implementation: Injectable<Interface> + 'static,
    {
        let interface_id: TypeId = TypeId::of::<Interface>();
        let erased_constructor: Constructor = |container: &Container| -> Box<dyn Any> {
            let target: Arc<Interface> = Implementation::__syringe_construct(container);
            Box::new(target)
        };

        let constructors = self.interface_constructor_registry
            .entry(interface_id)
            .or_insert_with(Vec::new);
        constructors.push(erased_constructor);
    }

    pub fn get<Interface: ?Sized + 'static>(&self) -> Arc<Interface> {
        let interface_id: TypeId = TypeId::of::<Interface>();
        let interface_name: &str = type_name::<Interface>();
        
        let interface_constructor_array: &Vec<Constructor> = self
            .interface_constructor_registry
            .get(&interface_id)
            .unwrap_or_else(||{
                panic!(
                    "Interface {:?} with TypeId {:?} not registered to Container",
                    interface_name,
                    interface_id
                )
            });

        if interface_constructor_array.len() != 1 {
            panic!(
                "Container::get() expects one implementation for {:?} but received {:?}",
                interface_name, 
                interface_constructor_array.len(),
            )
        }
        
        let interface_constructor = interface_constructor_array
            .first()
            .unwrap_or_else(||{
                panic!("Expected one implementation for {:?}",interface_name)
            });
        let any_box: Box<dyn Any> = interface_constructor(self);
        let boxed_arc: Box<Arc<Interface>> = any_box
            .downcast::<Arc<Interface>>()
            .unwrap_or_else(|boxed_any|{
                let any_interface_name: &str = type_name_of_val(&*boxed_any);
                panic!(
                    "Type mismatch during downcast. Expected: {:?} but received {:?}",
                    interface_name,
                    any_interface_name
                )
            });
        let arc_instance: Arc<Interface> = *boxed_arc;
        arc_instance
    }

    pub fn get_all<Interface: ?Sized + 'static>(&self) -> Vec<Arc<Interface>> {
        let interface_id: TypeId = TypeId::of::<Interface>();
        let interface_name: &str = type_name::<Interface>();
        
        let interface_constructor_array: &Vec<Constructor> = self
            .interface_constructor_registry
            .get(&interface_id)
            .unwrap_or_else(||{
                panic!(
                    "Interface {:?} with TypeId {:?} not registered to Container",
                    interface_name,
                    interface_id
                )
            });

        let all_instances: Vec<Arc<Interface>> = interface_constructor_array
            .iter()
            .map(|constructor: &Constructor| -> Arc<Interface> {
                let any_box: Box<dyn Any> = constructor(self);
                let boxed_arc: Box<Arc<Interface>> = any_box
                    .downcast::<Arc<Interface>>()
                    .unwrap_or_else(|boxed_any|{
                        let any_interface_name: &str = type_name_of_val(&*boxed_any);
                        panic!(
                            "Type mismatch during downcast. Expected: {:?} but received {:?}",
                            interface_name,
                            any_interface_name
                        )
                    });
                let arc_trait: Arc<Interface> = *boxed_arc;
                return arc_trait
            })
            .collect::<Vec<Arc<Interface>>>();
        
        return all_instances;
    }
}