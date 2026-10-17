use axum::routing::MethodRouter;

use crate::app_state::AppState;

pub struct RouteRegistration {
    pub path: &'static str,
    pub method_router: MethodRouter<AppState>,
}