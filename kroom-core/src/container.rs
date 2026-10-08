use std::any::{Any, TypeId, type_name, type_name_of_val};
use std::collections::HashMap;
use std::sync::Arc;

use crate::registration::Registration;
use crate::scope::Scope;
use crate::singleton_key::SingletonKey;

pub type Constructor = fn(&Container) -> Box<dyn Any>;

pub struct Container {
    type_to_registrations: HashMap<TypeId, Vec<Registration>>,
    type_to_singletons: HashMap<SingletonKey, Box<dyn Any>>,
}

impl Container {
    pub(crate) fn new() -> Self {
        Self {
            type_to_registrations: HashMap::new(),
            type_to_singletons: HashMap::new(),
        }
    }

    pub(crate) fn register_injectable(&mut self, registration: &Registration) {
        let interface_id: TypeId = registration.interface_id;
        let scope: Scope = (registration.scope)();

        self.type_to_registrations
            .entry(interface_id)
            .or_insert_with(Vec::new)
            .push(*registration);

        if scope == Scope::Singleton {
            self.register_singleton(registration);
        }
    }

    fn register_singleton(&mut self, registration: &Registration) {
        let singleton_key = SingletonKey::from_registration(registration);
        let constructor: Constructor = registration.constructor;
        let boxed_generic_injectable: Box<dyn Any> = constructor(self);
        self.type_to_singletons
            .insert(singleton_key, boxed_generic_injectable);
    }

    fn get_transient<Interface: ?Sized + 'static>(
        &self,
        registration: &Registration,
    ) -> Arc<Interface> {
        let constructor: Constructor = registration.constructor;
        let boxed_generic_injectable: Box<dyn Any> = constructor(self);
        let injectable: Arc<Interface> = self.extract_injectable(&boxed_generic_injectable);
        injectable
    }

    fn extract_injectable<Interface: ?Sized + 'static>(
        &self,
        boxed_generic_injectable: &Box<dyn Any>,
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

    fn get_singleton<Interface: ?Sized + 'static>(
        &self,
        registration: &Registration,
    ) -> Arc<Interface> {
        let singleton_key = SingletonKey::from_registration(registration);
        let boxed_generic_injectable = self
            .type_to_singletons
            .get(&singleton_key)
            .expect("Singleton to be registered");
        let injectable: Arc<Interface> = self.extract_injectable(boxed_generic_injectable);
        return injectable;
    }

    fn get_injectable_by_registration<Interface: ?Sized + 'static>(
        &self,
        registration: &Registration,
    ) -> Arc<Interface> {
        let scope: Scope = (registration.scope)();
        match &scope {
            Scope::Singleton => self.get_singleton(registration),
            Scope::Transient => self.get_transient(registration),
        }
    }

    pub fn get<Interface: ?Sized + 'static>(&self) -> Arc<Interface> {
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

    pub fn get_all<Interface: ?Sized + 'static>(&self) -> Vec<Arc<Interface>> {
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
