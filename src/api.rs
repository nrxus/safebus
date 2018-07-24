use client::{self, Client};

use rocket::{http::Status, response::status, State};
use rocket_contrib::Json;

use std::result::Result as StdResult;

type Result<T> = StdResult<Json<T>, status::Custom<String>>;

#[derive(FromForm, Debug, Clone, Copy, PartialEq)]
pub struct Area {
    pub lat: f64,
    pub lon: f64,
    pub lat_span: f64,
    pub lon_span: f64,
}

#[get("/bus_stops?<area>")]
fn bus_stops(client: State<Client>, area: Area) -> Result<Vec<client::BusStopInfo>> {
    client
        .bus_stops(area)
        .map(Json)
        .map_err(|e| status::Custom(Status::InternalServerError, e))
}

#[get("/bus_stop_status/<stop_id>")]
fn status(client: State<Client>, stop_id: String) -> Result<client::BusStopStatus> {
    client
        .bus_stop_status(stop_id.as_ref())
        .map(Json)
        .map_err(|e| status::Custom(Status::InternalServerError, e))
}
