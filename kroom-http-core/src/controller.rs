use axum::Router;

use crate::{app_state::AppState, route_registration::RouteRegistration};

pub trait Controller: Send + Sync {
    fn __kroom_http_route_registration(&self) -> Vec<RouteRegistration> {
        Vec::new()
    }

    fn __kroom_http_build_router(&self) -> Router<AppState> {
        let route_registrations: Vec<RouteRegistration> = self.__kroom_http_route_registration();
        let mut router: Router<AppState> = Router::new();
        for registration in route_registrations {
            let path = registration.path;
            let method_router = registration.method_router;
            router = router.route(&path, method_router);
        }
        router
    }
}