#![feature(proc_macro_hygiene, decl_macro)]

mod api;
mod client;

fn rocket() -> rocket::Rocket {
    let client = client::Client::new(reqwest::Client::new());
    rocket::ignite()
        .manage(client)
        .mount("/api", rocket::routes![api::bus_stops, api::status])
}

fn main() {
    rocket().launch();
}

#[cfg(test)]
mod api_test;
