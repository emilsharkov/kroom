use std::any::TypeId;

use crate::container::Constructor;
use crate::injectable::Injectable;
use crate::scope::Scope;

#[derive(Debug)]
pub struct Registration {
    pub interface_id: TypeId,
    pub constructor: Constructor,
    pub scope: fn() -> Scope,
    pub dependent_types: fn() -> Vec<TypeId>,
}

impl Registration {
    pub const fn of<Interface, Implementation>() -> Self
    where
        Interface: ?Sized + 'static,
        Implementation: Injectable<Interface> + 'static,
    {
        Self {
            interface_id: TypeId::of::<Interface>(),
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