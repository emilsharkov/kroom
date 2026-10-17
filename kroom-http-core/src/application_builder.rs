use std::{error::Error, sync::Arc};

use axum::Router;
use kroom_core::{container::Container, container_builder::ContainerBuilder};

use crate::{app_state::AppState, controller::Controller};

pub struct ApplicationBuilder {}

impl ApplicationBuilder {
    pub fn build() -> Result<Router, Box<dyn Error>> {
        let container: Container = ContainerBuilder::build()?;
        let controllers: Vec<Arc<dyn Controller>> = container.get_all::<dyn Controller>();

        let mut router: Router<AppState> = Router::new();
        for controller in controllers {
            let controller_router: Router<AppState> = controller.__kroom_http_build_router();
            router = router.merge(controller_router);
        }

        let app_state: AppState = AppState {
            container: Arc::new(container)
        };
        let app_router: Router = router.with_state(app_state);
        Ok(app_router)
    }
}