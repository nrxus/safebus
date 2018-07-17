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
fn bus_stops(area: Area, client: State<Client>) -> Result<Json<Vec<client::BusStop>>, String> {
    client.bus_stops(area).map(Json)
}

#[cfg(all(test, not(feature = "contract")))]
mod unit_test {
    use super::*;

    use mocktopus::mocking::{MockResult, Mockable};

    #[test]
    fn info_route() {
        let mut location = None;
        unsafe {
            Client::info.mock_raw(|_, loc| {
                location = Some(loc);
                MockResult::Return(Ok("Hello".to_string()))
            });
        }

        test::crime();
    }

    #[test]
    fn bus_stops() {
        let mut area = None;
        let expected = vec![client::BusStop {
            direction: String::from("S"),
            id: String::from("1_2345"),
            name: String::from("hello darkness"),
            lat: 1.23,
            lon: 123.23,
        }];
        unsafe {
            Client::bus_stops.mock_raw(|_, a| {
                area = Some(a);
                MockResult::Return(Ok(expected.clone()))
            });
        }

        let actual = test::bus_stops();
        assert_eq!(actual, expected);

        let area = area.expect("Client::bus_stops not called");
        assert_eq!(area.lat, 47.653435);
        assert_eq!(area.lon, -122.305641);
        assert_eq!(area.lat_span, 0.002);
        assert_eq!(area.lon_span, 0.003);
    }
}

#[cfg(all(test, feature = "contract"))]
mod contract_test {
    #[test]
    fn info_route() {
        super::test::crime();
    }

    #[test]
    fn bus_route() {
        let stops = super::test::bus_stops();
        assert!(stops.len() > 0);
    }
}

#[cfg(test)]
mod test {
    use client;
    use rocket::http::{ContentType, Status};
    use serde_json;

    pub fn bus_stops() -> Vec<client::BusStop> {
        let client = ::client();
        let mut response = client
            .get("/api/bus_stops?lat=47.653435&lon=-122.305641&lat_span=0.002&lon_span=0.003")
            .dispatch();
        assert_eq!(response.status(), Status::Ok);
        assert_eq!(response.content_type(), Some(ContentType::JSON));
        let body = response.body_string().expect("body was empty");
        serde_json::from_str(body.as_str())
            .expect("Could not parse api response into 'Vec<client::BusStop'")
    }

    pub fn crime() -> String {
        let client = ::client();
        let mut response = client
            .get("/api/info?latitude=-122.33&longitude=47.59")
            .dispatch();
        assert_eq!(response.status(), Status::Ok);
        assert_eq!(response.content_type(), Some(ContentType::JSON));
        response.body_string().expect("body was empty")
    }
}
