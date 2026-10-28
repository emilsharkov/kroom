use std::{error::Error, sync::Arc};

use axum::{Router, extract::Path, response::IntoResponse, routing::get};
use kroom_http_core::{application_builder::ApplicationBuilder, controller::Controller, route_registration::RouteRegistration};
use kroom_macros::injectable;
use tokio::net::TcpListener;

/*

#[controller]
struct GreetingController {
    #[inject]
    greeter: dyn Greeter
}

impl GreetingController {
    #[get("greet")]
    async fn greet(
        Json(input): Json<CreateUser>,
    ) -> impl IntoResponse {
        let name = input.name;
        self.greeter.greet(&name)
    }
}

-->

#[injectable(type = dyn Controller)]
struct GreetingController {
    #[inject]
    greeter: dyn Greeter
}

impl Controller for GreetingController {
    fn __kroom_http_route_registration(&self) -> Vec<RouteRegistration> {
        vec![
            {
                let greeter = Arc::clone(&self.greeter);
                RouteRegistration {
                    path: "/greet/{name}",
                    method_router: axum::routing::get(
                        move |
                            axum::extract::Json(input): axum::extract::Json<CreateUser>
                        | -> impl IntoResponse {
                            let greeter = Arc::clone(&greeter);
                            async move {
                                let name = input.name;
                                greeter.greet(&name)
                            }
                        },
                    ),
                }
            },
        ]
    }
}

*/

pub trait Greeter: Send + Sync {
    fn greet(&self, name: &str) -> String;
}

#[injectable(type = dyn Greeter)]
pub struct WorldGreeter;

impl Greeter for WorldGreeter {
    fn greet(&self, name: &str) -> String {
        format!("Hello, {}!", name)
    }
}

#[injectable(type = dyn Controller)]
pub struct GreetingController {
    #[inject]
    greeter: dyn Greeter,
}

impl GreetingController {
    async fn greet(
        &self,
        Path(name): Path<String>
    ) -> impl IntoResponse {
        self.greeter.greet(&name)
    }
}

impl Controller for GreetingController {
    fn __kroom_http_route_registration(&self) -> Vec<RouteRegistration> {
        vec![
            {
                let greeter = Arc::clone(&self.greeter);
                RouteRegistration {
                    path: "/greet",
                    method_router: axum::routing::get(move || {
                        let greeter = Arc::clone(&greeter);
                        async move {
                            greeter.greet("World")
                        }
                    }),
                }
            },
            {
                let greeter = Arc::clone(&self.greeter);
                RouteRegistration {
                    path: "/greet/{name}",
                    method_router: axum::routing::get(move |
                        Path(name): Path<String>
                    | {
                        let greeter = Arc::clone(&greeter);
                        async move {
                            greeter.greet(&name)
                        }
                    }),
                }
            },
        ]
    }
}


#[tokio::main]
async fn main() -> Result<(),Box<dyn Error>> {
    let app: Router = ApplicationBuilder::build()?;
    let listener: TcpListener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    axum::serve(listener, app).await?;
    Ok(())
}