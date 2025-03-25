#[macro_use]
extern crate rocket;

pub mod controller;
pub mod model;
pub mod repository;
pub mod service;

use crate::controller::route_stage;
use dotenvy::dotenv;

#[launch]
fn rocket() -> _ {
    dotenv().ok();
    rocket::build()
        .manage(reqwest::Client::builder().build().unwrap())
        .attach(route_stage())
}
