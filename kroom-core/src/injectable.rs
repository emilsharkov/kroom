use std::sync::Arc;

use crate::container::Container;

pub trait Injectable<Interface>
where 
    Interface: ?Sized + 'static, 
{
    fn __kroom_construct(
        container: &Container
    ) -> Arc<Interface>;
}