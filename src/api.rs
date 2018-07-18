use client::{self, Client};

use rocket::State;
use rocket_contrib::Json;

#[derive(FromForm, Debug, Clone, Copy, PartialEq)]
pub struct Location {
    pub latitude: f64,
    pub longitude: f64,
}

#[derive(FromForm, Debug, Clone, Copy, PartialEq)]
pub struct Area {
    pub lat: f64,
    pub lon: f64,
    pub lat_span: f64,
    pub lon_span: f64,
}

#[get("/info?<location>")]
fn info(location: Location, client: State<Client>) -> Result<Json<String>, String> {
    client.info(location).map(Json)
}

#[get("/bus_stops?<area>")]
fn bus_stops(area: Area, client: State<Client>) -> Result<Json<Vec<client::BusStopInfo>>, String> {
    client.bus_stops(area).map(Json)
}
