#![feature(proc_macro_hygiene, decl_macro)]

// Dependencies
extern crate chrono;
extern crate geo_types;
extern crate geojson;
extern crate reqwest;
#[macro_use]
extern crate rocket;
extern crate rocket_contrib;
#[macro_use]
extern crate serde;

// Unit Test Dependencies
#[cfg(all(test, not(feature = "integration")))]
#[macro_use]
extern crate approx;
#[cfg(all(test, not(feature = "integration")))]
extern crate mockito;
#[cfg(all(test, not(feature = "integration")))]
extern crate mocktopus;
#[cfg(all(test, not(feature = "integration")))]
extern crate serde_urlencoded;
#[cfg(all(test, not(feature = "integration")))]
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
