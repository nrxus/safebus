use crate::client::{self, Client};

use rocket::{http::Status, request::Form, response::status, State};
use rocket_contrib::json::Json;

type ApiResult<T> = Result<Json<T>, status::Custom<String>>;

#[derive(rocket::FromForm, Debug, Clone, Copy, PartialEq)]
pub struct Area {
    pub lat: f64,
    pub lon: f64,
    pub lat_span: f64,
    pub lon_span: f64,
    pub limit: Option<u16>,
}

#[rocket::get("/bus_stops?<area..>")]
pub fn bus_stops(
    client: State<'_, Client>,
    area: Form<Area>,
) -> ApiResult<Vec<client::BusStopInfo>> {
    client.bus_stops(&*area).into_api()
}

#[rocket::get("/bus_stop_status/<stop_id>")]
pub fn status(client: State<'_, Client>, stop_id: String) -> ApiResult<client::BusStopStatus> {
    client.bus_stop_status(stop_id.as_ref()).into_api()
}

trait IntoApiResult<T> {
    fn into_api(self) -> ApiResult<T>;
}

impl<T> IntoApiResult<T> for Result<T, String> {
    fn into_api(self) -> ApiResult<T> {
        self.map(Json)
            .map_err(|e| status::Custom(Status::InternalServerError, e))
    }
}
