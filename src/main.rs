pub mod model;

// use actix_web::{get, App, HttpResponse, HttpServer, Responder};
use model::Event;
use statig::awaitable::IntoStateMachineExt;

#[tokio::main]
async fn main() {
    let mut led = model::Lamp::default().state_machine();
    led.handle(&Event::TurnOn).await;
    led.handle(&Event::TurnOff).await;
    led.handle(&Event::TurnOn).await;
}
// #[get("/")]
// async fn hello_world() -> impl Responder {
//     // let mut sm = sm.init().await;
//     let mut led = model::Lamp::default().state_machine();
//     led.handle(&Event::TurnOn).await;
//     HttpResponse::Ok().body(format!("Hi! {}", led.led))
// }

// #[actix_web::main]
// async fn main() -> std::io::Result<()> {

//     // let mut state_m = led.init().await;
//     // let mut sm = Arc::new(state_m);
//     HttpServer::new(move || {
//         // let led: statig::awaitable::UninitializedStateMachine<Lamp> = model::Lamp::default().uninitialized_state_machine();
//         App::new()
//             // .app_data(web::Data::new(led))
//             .service(hello_world)
//     })
//     .bind(("127.0.0.1", 8080))?
//     .run()
//     .await
// }
