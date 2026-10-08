use std::any::TypeId;

use crate::registration::Registration;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SingletonKey {
    pub interface_id: TypeId,
    pub implementation_id: TypeId,
}

impl SingletonKey {
    pub fn from_registration(registration: &Registration) -> Self {
        Self {
            interface_id: registration.interface_id,
            implementation_id: registration.implementation_id,
        }
    }
}
