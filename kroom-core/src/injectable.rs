use std::{any::TypeId, sync::Arc};

use crate::{container::Container, scope::Scope};

pub trait Injectable<Interface>
where 
    Interface: ?Sized + 'static, 
{
    fn __kroom_construct(
        container: &Container
    ) -> Arc<Interface>;

    fn __kroom_scope() -> Scope;
    
    fn __kroom_dependent_types() -> Vec<TypeId>;
    
    // fn __kroom_scope() -> &'static Scope {
    //     &Scope::Singleton
    // }
    
    // fn __kroom_dependencies() -> &'static [TypeId] {
    //     &[]
    // }
}