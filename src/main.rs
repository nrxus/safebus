#![feature(proc_macro_hygiene, decl_macro)]

mod api;
mod client;

fn rocket(client: client::Client) -> rocket::Rocket {
    rocket::ignite()
        .manage(client)
        .mount("/api", rocket::routes![api::bus_stops, api::status])
}

fn main() {
    let client = client::Client::from_http_client(reqwest::Client::new());
    rocket(client).launch();
}

#[cfg(test)]
mod api_test;
