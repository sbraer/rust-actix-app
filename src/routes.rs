use actix_web::{get, HttpResponse};
use std::ops::Deref;

use crate::errors::AppResponse;
use crate::person::{Person, RandomGenerator};

#[get("/health")]
async fn health() -> HttpResponse {
    HttpResponse::Ok().body("OK")
}

#[get("/api/random_persons")]
async fn random_persons() -> AppResponse {
    let items: Vec<Person> = (0..100).map(|_| Person::random()).collect();
    Ok(HttpResponse::Ok().json(items.deref()))
}
