#![feature(plugin)]
#![plugin(rocket_codegen)]
#![feature(custom_derive)]
#![cfg_attr(
    all(test, not(feature = "contract")),
    feature(proc_macro, proc_macro_mod, proc_macro_gen)
)]

// Dependencies
extern crate chrono;
#[macro_use]
extern crate hyper;
extern crate geo_types;
extern crate geojson;
extern crate reqwest;
extern crate rocket;
extern crate rocket_contrib;
extern crate serde;
#[macro_use]
extern crate serde_derive;

// Unit Test Dependencies
#[cfg(all(test, not(feature = "contract")))]
#[macro_use]
extern crate approx;
#[cfg(all(test, not(feature = "contract")))]
extern crate mockito;
#[cfg(all(test, not(feature = "contract")))]
extern crate mocktopus;
#[cfg(all(test, not(feature = "contract")))]
extern crate serde_urlencoded;
#[cfg(all(test, not(feature = "contract")))]
extern crate url;

// Test Dependencies
#[cfg(test)]
extern crate serde_json;

mod api;

mod client;

fn rocket() -> rocket::Rocket {
    let client = client::Client::new(reqwest::Client::new());
    rocket::ignite()
        .manage(client)
        .mount("/api", routes![api::bus_stops, api::status])
}

fn main() {
    rocket().launch();
}

#[cfg(test)]
mod api_test;
