mod errors;
mod person;
mod routes;

use std::sync::RwLock;

use actix_web::{middleware::Logger, web, App, HttpServer};
use routes::{add_person, delete_person, health, persons, single_person, update_person, AppState};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "info");
    std::env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();

    let shared_state = web::Data::new(AppState {
        person_collection: RwLock::new(person::create_person_collection()),
    });

    HttpServer::new(move || {
        let logger = Logger::default();
        App::new()
            .wrap(logger)
            .app_data(shared_state.clone())
            .service(single_person)
            .service(persons)
            .service(add_person)
            .service(delete_person)
            .service(update_person)
            .service(health)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
