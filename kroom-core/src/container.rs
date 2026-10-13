use std::any::{Any, TypeId, type_name, type_name_of_val};
use std::collections::HashMap;
use std::sync::Arc;

use crate::registration::Registration;
use crate::scope::Scope;

pub type Constructor = fn(&Container) -> Box<dyn Send + Sync + Any>;

pub struct Container {
    type_to_registrations: HashMap<TypeId, Vec<Registration>>,
    interface_to_implementation_singleton: HashMap<TypeId, HashMap<TypeId, Box<dyn Send + Sync + Any>>>,
}

impl Container {
    pub(crate) fn new() -> Self {
        Self {
            type_to_registrations: HashMap::new(),
            interface_to_implementation_singleton: HashMap::new(),
        }
    }

    pub(crate) fn register_injectable(&mut self, implementation_id: &TypeId, registration: &Registration) {
        self.type_to_registrations
            .entry(*implementation_id)
            .or_insert_with(Vec::new)
            .push(*registration);
    }

    pub(crate) fn register_singleton(&mut self, registration: &Registration) {
        let interface_id: TypeId = registration.interface_id;
        let implementation_id: TypeId = registration.implementation_id;
        let constructor: Constructor = registration.constructor;
        let boxed_generic_injectable: Box<dyn Send + Sync + Any> = constructor(self);
        self.interface_to_implementation_singleton
            .entry(interface_id)
            .or_insert_with(HashMap::new)
            .insert(implementation_id, boxed_generic_injectable);
    }

    fn get_transient<Interface: Send + Sync + ?Sized + 'static>(
        &self,
        registration: &Registration,
    ) -> Arc<Interface> {
        let constructor: Constructor = registration.constructor;
        let boxed_generic_injectable: Box<dyn Send + Sync + Any> = constructor(self);
        let injectable: Arc<Interface> = self.extract_injectable(&boxed_generic_injectable);
        injectable
    }

    fn extract_injectable<Interface: Send + Sync + ?Sized + 'static>(
        &self,
        boxed_generic_injectable: &Box<dyn Send + Sync + Any>,
    ) -> Arc<Interface> {
        let arc_injectable = boxed_generic_injectable
            .downcast_ref::<Arc<Interface>>()
            .unwrap_or_else(|| {
                let interface_name: &str = type_name::<Interface>();
                let any_interface_name: &str = type_name_of_val(&**boxed_generic_injectable);
                panic!(
                    "Type mismatch downcasting singleton. Expected: {:?} but received {:?}",
                    interface_name, any_interface_name
                )
            });
        return Arc::clone(arc_injectable);
    }

    fn get_singleton<Interface: Send + Sync + ?Sized + 'static>(
        &self,
        registration: &Registration,
    ) -> Arc<Interface> {
        let interface_id: TypeId = registration.interface_id;
        let implementation_id: TypeId = registration.implementation_id;

        let boxed_generic_injectable = self
            .interface_to_implementation_singleton
            .get(&interface_id)
            .expect("Interface to have at least one registered singleton")
            .get(&implementation_id)
            .expect("Implementation to have a singleton");
        let injectable: Arc<Interface> = self.extract_injectable(boxed_generic_injectable);
        return injectable;
    }

    fn get_injectable_by_registration<Interface: Send + Sync + ?Sized + 'static>(
        &self,
        registration: &Registration,
    ) -> Arc<Interface> {
        let scope: Scope = (registration.scope)();
        match &scope {
            Scope::Singleton => self.get_singleton(registration),
            Scope::Transient => self.get_transient(registration),
        }
    }

    pub fn get<Interface: Send + Sync + ?Sized + 'static>(&self) -> Arc<Interface> {
        let interface_id: TypeId = TypeId::of::<Interface>();
        let interface_name: &str = type_name::<Interface>();

        let registrations: &Vec<Registration> = self
            .type_to_registrations
            .get(&interface_id)
            .unwrap_or_else(|| {
                panic!(
                    "Interface {:?} with TypeId {:?} not registered to Container",
                    interface_name, interface_id
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
            .unwrap_or_else(|| panic!("Expected one implementation for {:?}", interface_name));

        let injectable: Arc<Interface> = self.get_injectable_by_registration(registration);
        injectable
    }

    pub fn get_all<Interface: Send + Sync + ?Sized + 'static>(&self) -> Vec<Arc<Interface>> {
        let interface_id: TypeId = TypeId::of::<Interface>();
        let interface_name: &str = type_name::<Interface>();

        let registrations: &Vec<Registration> = self
            .type_to_registrations
            .get(&interface_id)
            .unwrap_or_else(|| {
                panic!(
                    "Interface {:?} with TypeId {:?} not registered to Container",
                    interface_name, interface_id
                )
            });

        let all_instances: Vec<Arc<Interface>> = registrations
            .iter()
            .map(|registration: &Registration| -> Arc<Interface> {
                let injectable: Arc<Interface> = self.get_injectable_by_registration(registration);
                injectable
            })
            .collect::<Vec<Arc<Interface>>>();

        all_instances
    }
}
