use crate::container::Container;

pub trait Injectable {
    type Target: ?Sized + 'static;
    
    fn __syringe_construct(
        container: &Container
    ) -> Box<Self::Target>;
}