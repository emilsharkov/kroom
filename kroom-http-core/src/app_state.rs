use std::sync::Arc;

use kroom_core::container::Container;

#[derive(Clone)]
pub struct AppState {
    pub container: Arc<Container>
}