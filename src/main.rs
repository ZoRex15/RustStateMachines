use std::sync::Arc;

// Replace some of the `axum::` types with `aide::axum::` ones.
use aide::{
    axum::{
        routing::{get, post},
        ApiRouter, IntoApiResponse,
    },
    openapi::{Info, OpenApi},
};
use axum::{extract::State, response::sse::Event, Extension, Json};
use schemars::JsonSchema;
use serde::Deserialize;
use tokio::sync::Mutex;


mod model;

// We'll need to derive `JsonSchema` for
// all types that appear in the api documentation.
#[derive(Deserialize, JsonSchema)]
struct User {
    name: String,
}

#[derive(Deserialize, JsonSchema)]
struct Command {
    id: i64
    // power: String
}

async fn hello_user(Json(user): Json<User>) -> impl IntoApiResponse {
    format!("hello {}", user.name)
}

// #[axum::debug_handler]
async fn turn_on(
    State(lamp): State<Arc<Mutex<model::Lamp>>>,
    Json(command): Json<Command>
) -> impl IntoApiResponse {
    let event = model::Event::Switch;
    let mut lamp = lamp.lock().await;
    let _ = lamp.on(&event);
    format!("lamp state is {}", lamp.led)
}

// async fn turn_off(Json(command): Json<Command>) -> impl IntoApiResponse {
    
// }

// Note that this clones the document on each request.
// To be more efficient, we could wrap it into an Arc,
// or even store it as a serialized string.
async fn serve_api(Extension(api): Extension<OpenApi>) -> impl IntoApiResponse {
    Json(api)
}

#[tokio::main]
async fn main() {
    let state_machine = model::Lamp::default();
    let state = Arc::new(Mutex::new(state_machine));
    let app = ApiRouter::new()
        // Change `route` to `api_route` for the route
        // we'd like to expose in the documentation.
        .api_route("/hello", post(hello_user))
        .api_route("/on", post(turn_on))
        // We'll serve our generated document here.
        .with_state(state)
        .route("/api.json", get(serve_api));

    let mut api = OpenApi {
        info: Info {
            description: Some("an example API".to_string()),
            ..Info::default()
        },
        ..OpenApi::default()
    };

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();

    axum::serve(
        listener,
        app
            // Generate the documentation.
            .finish_api(&mut api)
            // Expose the documentation to the handlers.
            .layer(Extension(api))
            .into_make_service(),
    )
    .await
    .unwrap();
}