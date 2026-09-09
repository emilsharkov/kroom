use std::collections::HashMap;
use std::any::{Any, TypeId, type_name, type_name_of_val};
use std::sync::Arc;
use crate::injectable::Injectable;
use crate::registration::Registration;

pub type Constructor = fn(&Container) -> Box<dyn Any>;

pub struct Container {
    interface_constructor_registry: HashMap<TypeId, Vec<Constructor>>,
}

impl Container {
    pub fn new() -> Self {
        Self {
            interface_constructor_registry: HashMap::new(),
        }
    }

    pub fn auto_register(&mut self) {
        for registration in inventory::iter::<Registration> {
            let Registration { interface_id, constructor } = registration;
            self.register_inner(interface_id, constructor);
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
        self.register_inner(&interface_id, &erased_constructor);
    }

    pub fn register_inner(&mut self, interface_id: &TypeId, constructor: &Constructor) {
        let constructors = self.interface_constructor_registry
            .entry(*interface_id)
            .or_insert_with(Vec::new);
        constructors.push(*constructor);
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
        
        all_instances
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    trait Greeter: Send + Sync {
        fn greet(&self) -> String;
    }

    struct EnglishGreeter;
    impl Greeter for EnglishGreeter {
        fn greet(&self) -> String {
            "Hello".to_string()
        }
    }

    impl Injectable<EnglishGreeter> for EnglishGreeter {
        fn __syringe_construct(_container: &Container) -> Arc<EnglishGreeter> {
            Arc::new(EnglishGreeter)
        }
    }

    impl Injectable<dyn Greeter> for EnglishGreeter {
        fn __syringe_construct(_container: &Container) -> Arc<dyn Greeter> {
            Arc::new(EnglishGreeter)
        }
    }

    struct SpanishGreeter;
    impl Greeter for SpanishGreeter {
        fn greet(&self) -> String {
            "Hola".to_string()
        }
    }

    impl Injectable<dyn Greeter> for SpanishGreeter {
        fn __syringe_construct(_container: &Container) -> Arc<dyn Greeter> {
            Arc::new(SpanishGreeter)
        }
    }

    struct GreeterService {
        greeter: Arc<dyn Greeter>,
    }

    impl Injectable<GreeterService> for GreeterService {
        fn __syringe_construct(container: &Container) -> Arc<GreeterService> {
            let greeter = container.get::<dyn Greeter>();
            Arc::new(GreeterService { greeter })
        }
    }

    #[test]
    fn test_register_and_get_concrete() {
        let mut container = Container::new();
        container.register::<EnglishGreeter, EnglishGreeter>();

        let greeter = container.get::<EnglishGreeter>();
        assert_eq!(greeter.greet(), "Hello");
    }

    #[test]
    fn test_register_and_get_interface() {
        let mut container = Container::new();
        container.register::<dyn Greeter, EnglishGreeter>();

        let greeter = container.get::<dyn Greeter>();
        assert_eq!(greeter.greet(), "Hello");
    }

    #[test]
    fn test_get_all_multiple_registrations() {
        let mut container = Container::new();
        container.register::<dyn Greeter, EnglishGreeter>();
        container.register::<dyn Greeter, SpanishGreeter>();

        let greeters = container.get_all::<dyn Greeter>();
        assert_eq!(greeters.len(), 2);
        assert_eq!(greeters[0].greet(), "Hello");
        assert_eq!(greeters[1].greet(), "Hola");
    }

    #[test]
    fn test_get_all_single_registration() {
        let mut container = Container::new();
        container.register::<dyn Greeter, SpanishGreeter>();

        let greeters = container.get_all::<dyn Greeter>();
        assert_eq!(greeters.len(), 1);
        assert_eq!(greeters[0].greet(), "Hola");
    }

    #[test]
    fn test_nested_dependency_resolution() {
        let mut container = Container::new();
        container.register::<dyn Greeter, EnglishGreeter>();
        container.register::<GreeterService, GreeterService>();

        let service = container.get::<GreeterService>();
        assert_eq!(service.greeter.greet(), "Hello");
    }

    #[test]
    #[should_panic(expected = "not registered to Container")]
    fn test_get_unregistered_panics() {
        let container = Container::new();
        let _ = container.get::<dyn Greeter>();
    }

    #[test]
    #[should_panic(expected = "not registered to Container")]
    fn test_get_all_unregistered_panics() {
        let container = Container::new();
        let _ = container.get_all::<dyn Greeter>();
    }

    #[test]
    #[should_panic(expected = "Container::get() expects one implementation")]
    fn test_get_multiple_registered_panics() {
        let mut container = Container::new();
        container.register::<dyn Greeter, EnglishGreeter>();
        container.register::<dyn Greeter, SpanishGreeter>();

        let _ = container.get::<dyn Greeter>();
    }
}