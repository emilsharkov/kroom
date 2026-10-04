use std::sync::Arc;

use crate::{container::Container, registration::RegisteredType, scope::Scope};

pub trait Injectable<Interface>
where 
    Interface: ?Sized + 'static, 
{
    fn __kroom_construct(
        container: &Container
    ) -> Arc<Interface>;

    fn __kroom_scope() -> Scope;
    
    fn __kroom_dependent_types() -> Vec<RegisteredType>;

    fn __kroom_interface_name() -> String;
    
    fn __kroom_implementation_name() -> String;
}