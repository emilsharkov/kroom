use std::{error::Error, sync::Arc};

use axum::routing::method_routing::MethodRouter;
use axum::Router;
use kroom_core::{container::Container, container_builder::ContainerBuilder, injectable::Injectable};

/*

#[controller]
struct GreetingController {
    #[inject]
    greeter: dyn Greeter
}

-->

trait Controller {
    fn build_router(container: &Container) -> Router<Container>;
}

#[injectable(type = dyn Controller)]
struct GreetingController {
    #[inject]
    greeter: dyn Greeter
}

*/

pub struct RouteRegistration {
    pub path: String,
    pub method_router: MethodRouter<AppState>,
}

pub trait Controller: Send + Sync {
    fn __kroom_http_route_registration(&self) -> Vec<RouteRegistration> {
        Vec::new()
    }

    fn __kroom_http_build_router(&self) -> Router<AppState> {
        let route_registrations: Vec<RouteRegistration> = self.__kroom_http_route_registration();
        let mut router: Router<AppState> = Router::new();
        for registration in route_registrations {
            let path: String = registration.path;
            let method_router = registration.method_router;
            router = router.route(&path, method_router);
        }
        router
    }
}

#[derive(Clone)]
pub struct AppState {
    container: Arc<Container>
}

#[tokio::main]
async fn main() -> Result<(),Box<dyn Error>> {
    let container: Container = ContainerBuilder::build()?;
    let controllers: Vec<Arc<dyn Controller>> = container.get_all::<dyn Controller>();

    let mut big_router: Router<AppState> = Router::new();
    for controller in controllers {
        let controller_router: Router<AppState> = controller.__kroom_http_build_router();
        big_router = big_router.merge(controller_router);
    }

    let app_state: AppState = AppState {
        container: Arc::new(container)
    };
    let router: Router = big_router.with_state(app_state);
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    axum::serve(listener, router).await?;
    Ok(())
}