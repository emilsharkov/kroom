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
    
    fn __kroom_dependent_types() -> Vec<(TypeId,String)>;

    fn __kroom_interface_name() -> String;
    
    fn __kroom_implementation_name() -> String;
}