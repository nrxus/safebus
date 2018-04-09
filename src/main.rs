#![feature(plugin)]
#![plugin(rocket_codegen)]
#![feature(custom_derive)]

extern crate chrono;
extern crate reqwest;
extern crate rocket;
#[macro_use]
extern crate hyper;

mod api;
mod client;
mod query;
mod service;

fn rocket() -> rocket::Rocket {
    rocket::ignite()
        .manage(reqwest::Client::new())
        .mount("/api", routes![api::info])
}

fn main() {
    rocket().launch();
}

#[cfg(test)]
fn client() -> rocket::local::Client {
    rocket::local::Client::new(rocket()).unwrap()
}
