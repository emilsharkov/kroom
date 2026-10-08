use std::any::TypeId;
use std::fmt;

use crate::container::Constructor;
use crate::injectable::Injectable;
use crate::scope::Scope;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RegisteredType {
    pub id: TypeId,
    pub name: String,
}

#[derive(Copy, Clone)]
pub struct Registration {
    pub interface_id: TypeId,
    pub interface_name: fn() -> String,
    pub implementation_id: TypeId,
    pub implementation_name: fn() -> String,
    pub constructor: Constructor,
    pub scope: fn() -> Scope,
    pub dependent_types: fn() -> Vec<RegisteredType>,
}

impl fmt::Debug for Registration {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Registration")
            .field("interface_id", &self.interface_id)
            .field("interface_name", &(self.interface_name)())
            .field("implementation_id", &self.implementation_id)
            .field("implementation_name", &(self.implementation_name)())
            .field("constructor", &self.constructor)
            .field("scope", &(self.scope)())
            .field("dependent_types", &(self.dependent_types)())
            .finish()
    }
}

impl Registration {
    pub const fn of<Interface, Implementation>() -> Self
    where
        Interface: ?Sized + 'static,
        Implementation: Injectable<Interface> + 'static,
    {
        Self {
            interface_id: TypeId::of::<Interface>(),
            interface_name: Implementation::__kroom_interface_name,
            implementation_id: TypeId::of::<Implementation>(),
            implementation_name: Implementation::__kroom_implementation_name,
            constructor: |container| {
                let target = Implementation::__kroom_construct(container);
                Box::new(target)
            },
            scope: Implementation::__kroom_scope,
            dependent_types: Implementation::__kroom_dependent_types,
        }
    }
}

inventory::collect!(Registration);
