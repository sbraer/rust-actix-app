use actix_web::{get, HttpResponse};
use std::ops::Deref;

use crate::person::{Person, RandomGenerator};

#[get("/health")]
async fn health() -> HttpResponse {
    HttpResponse::Ok().body("OK")
}

#[get("/api/random_persons")]
async fn random_persons() -> HttpResponse {
    let items: Vec<Person> = (0..100).map(|_| Person::random()).collect();
    HttpResponse::Ok().json(items.deref())
}
