#![feature(plugin)]
#![plugin(rocket_codegen)]
#![feature(custom_derive)]

extern crate chrono;
#[macro_use]
extern crate hyper;
extern crate reqwest;
extern crate rocket;

#[cfg(all(test, not(feature = "contract")))]
extern crate mockito;

#[cfg(all(test, not(feature = "contract")))]
extern crate url;

mod api;
mod client;

fn rocket() -> rocket::Rocket {
    let client = client::Client::new(reqwest::Client::new());
    rocket::ignite()
        .manage(client)
        .mount("/api", routes![api::info])
}

fn main() {
    rocket().launch();
}

#[cfg(test)]
fn client() -> rocket::local::Client {
    rocket::local::Client::new(rocket()).unwrap()
}
