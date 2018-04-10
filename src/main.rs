#![feature(plugin)]
#![plugin(rocket_codegen)]
#![feature(custom_derive)]

extern crate chrono;
#[cfg_attr(any(not(test), feature = "contract"), macro_use)]
extern crate hyper;
extern crate reqwest;
extern crate rocket;

mod api;
mod client;
mod query;
mod service;

trait RocketExt: Sized {
    fn inject(self) -> Self;
}

#[cfg(any(not(test), feature = "contract"))]
impl RocketExt for rocket::Rocket {
    fn inject(self) -> Self {
        self.manage(client::CrimeClient::new())
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
