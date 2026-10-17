use std::{error::Error, sync::Arc};

use axum::Router;
use kroom_http_core::{application_builder::ApplicationBuilder, controller::Controller, route_registration::RouteRegistration};
use kroom_macros::injectable;
use tokio::net::TcpListener;

/*

#[controller]
struct GreetingController {
    #[inject]
    greeter: dyn Greeter

    #[get("greet")]
    async fn greet(
        Path(id): Path<u64>,
        Json(input): Json<CreateUser>,
        Query(params): Query<HashMap<String, String>>
    ) -> impl IntoResponse {
        let name = input.name;
        let greeting = self.greeter.greet(&name);
        greeting
    }
}

-->

#[injectable(type = dyn Controller)]
struct GreetingController {
    #[inject]
    greeter: dyn Greeter
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
                    method_router: axum::routing::get(
                        move |
                            axum::extract::Path(name): axum::extract::Path<String>
                        | {
                            let greeter = Arc::clone(&greeter);
                            async move {
                                greeter.greet(&name)
                            }
                        },
                    ),
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