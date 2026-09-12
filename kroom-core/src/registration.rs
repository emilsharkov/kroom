use std::any::TypeId;

use crate::container::Constructor;
use crate::injectable::Injectable;

#[derive(Debug)]
pub struct Registration {
    pub interface_id: TypeId,
    pub constructor: Constructor,
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
        }
    }
}

inventory::collect!(Registration);