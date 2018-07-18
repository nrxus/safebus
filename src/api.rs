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
fn info(client: State<Client>, location: Location) -> Result<Json<String>, String> {
    client.info(location).map(Json)
}

#[get("/bus_stops?<area>")]
fn bus_stops(client: State<Client>, area: Area) -> Result<Json<Vec<client::BusStopInfo>>, String> {
    client.bus_stops(area).map(Json)
}

//https://gisrevprxy.seattle.gov/ArcGIS/rest/services/DoIT_ext/SP_Precincts_Beats/MapServer/2/query?geometry=-122.2861032,47.6828274&geometryType=esriGeometryPoint&inSR=4326&returnGeometry=false&f=pjson

//https://data.seattle.gov/resource/xurz-654a.json?$where=occ_datetime%3E%272018-07-13T00:00:00%27%20AND%20beat=%27U3%27

#[get("/bus_stop_status/<stop_id>")]
fn status(client: State<Client>, stop_id: String) -> Result<Json<client::BusStopStatus>, String> {
    client.bus_stop_status(stop_id).map(Json)
}
