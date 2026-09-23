use std::any::{TypeId};

use crate::container::Constructor;
use crate::injectable::Injectable;
use crate::scope::Scope;

#[derive(Debug)]
pub struct Registration {
    pub interface_id: TypeId,
    pub implementation_id: TypeId,
    pub implementation_name: fn() -> String,
    pub constructor: Constructor,
    pub scope: fn() -> Scope,
    pub dependent_types: fn() -> Vec<(TypeId,String)>,
}

impl Registration {
    pub const fn of<Interface, Implementation>() -> Self
    where
        Interface: ?Sized + 'static,
        Implementation: Injectable<Interface> + 'static,
    {
        Self {
            interface_id: TypeId::of::<Interface>(),
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