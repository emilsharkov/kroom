use std::collections::HashMap;
use std::any::{Any, TypeId, type_name, type_name_of_val};
use std::error::Error;
use std::sync::Arc;
use crate::container_validator::ContainerValidator;
use crate::injectable::Injectable;
use crate::registration::Registration;
use crate::scope::Scope;

pub type Constructor = fn(&Container) -> Box<dyn Any>;

pub struct Container {
    type_to_registrations: HashMap<TypeId, Vec<Registration>>,
    type_to_singletons: HashMap<TypeId, Vec<Arc<dyn Any>>>,
}

impl Container {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        ContainerValidator::new().resolve()?;
        let container: Container = Self {
            type_to_registrations: HashMap::new(),
            type_to_singletons: HashMap::new()
        };
        Ok(container)
    }

    fn create_injectable<Interface: ?Sized + 'static> (
        &self, 
        registration: &Registration
    ) -> Arc<Interface> {
        let interface_name: &str = type_name::<Interface>();
        let constructor: Constructor = registration.constructor;
        let boxed_generic_injectable: Box<dyn Any> = constructor(self);
        let boxed_arc_injectable: Box<Arc<Interface>> = boxed_generic_injectable
            .downcast::<Arc<Interface>>()
            .unwrap_or_else(|boxed_any|{
                let any_interface_name: &str = type_name_of_val(&*boxed_any);
                panic!(
                    "Type mismatch during downcast. Expected: {:?} but received {:?}",
                    interface_name,
                    any_interface_name
                )
            });
        let arc_injectable: Arc<Interface> = *boxed_arc_injectable;
        arc_injectable
    } 

    fn get_singleton<Interface: ?Sized + 'static>(
        &self,
        registration: &Registration
    ) -> Arc<Interface> {
        let interface_id: TypeId = registration.interface_id;
        let implementation_id: TypeId = registration.implementation_id;
        if let Some(generic_injectables) = self.type_to_singletons.get(&interface_id) {

        } else {
            let arc_injectable: Arc<Interface> = self.create_injectable(registration);
            self.type_to_singletons
                .entry(interface_id)
                .or_default()
                .push(arc_injectable);
            return arc_injectable;
        }
    }

    pub fn get<Interface: ?Sized + 'static>(&self) -> Arc<Interface> {
        let interface_id: TypeId = TypeId::of::<Interface>();
        let interface_name: &str = type_name::<Interface>();
        
        let registrations: &Vec<Registration> = self
            .type_to_registrations
            .get(&interface_id)
            .unwrap_or_else(||{
                panic!(
                    "Interface {:?} with TypeId {:?} not registered to Container",
                    interface_name,
                    interface_id
                )
            });

        if registrations.len() != 1 {
            panic!(
                "Container::get() expects one implementation for {:?} but received {:?}",
                interface_name, 
                registrations.len(),
            )
        }
        
        let registration = registrations
            .first()
            .unwrap_or_else(||{
                panic!("Expected one implementation for {:?}",interface_name)
            });
        
        let scope: Scope = (registration.scope)();
        match &scope {
            Scope::Singleton => self.get_singleton(registration),
            Scope::Transient => self.create_injectable(registration)
        }
    }

    pub fn get_all<Interface: ?Sized + 'static>(&self) -> Vec<Arc<Interface>> {
        let interface_id: TypeId = TypeId::of::<Interface>();
        let interface_name: &str = type_name::<Interface>();
        
        let registrations: &Vec<Registration> = self
            .type_to_registrations
            .get(&interface_id)
            .unwrap_or_else(||{
                panic!(
                    "Interface {:?} with TypeId {:?} not registered to Container",
                    interface_name,
                    interface_id
                )
            });

        let all_instances: Vec<Arc<Interface>> = registrations
            .iter()
            .map(|registration: &Registration| -> Arc<Interface> {
                let scope: Scope = (registration.scope)();
                match &scope {
                    Scope::Singleton => self.get_singleton(registration),
                    Scope::Transient => self.create_injectable(registration)
                }
            })
            .collect::<Vec<Arc<Interface>>>();
        
        all_instances
    }
}