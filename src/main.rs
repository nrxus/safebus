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
mod providers;
mod query;
mod service;

trait RocketExt: Sized {
    fn inject(self) -> Self;
}

impl RocketExt for rocket::Rocket {
    fn inject(self) -> Self {
        let crime_client = providers::crime_client();
        self.manage(crime_client)
    }
}

fn rocket() -> rocket::Rocket {
    rocket::ignite().inject().mount("/api", routes![api::info])
}

fn main() {
    rocket().launch();
}

#[cfg(test)]
fn client() -> rocket::local::Client {
    rocket::local::Client::new(rocket()).unwrap()
}
