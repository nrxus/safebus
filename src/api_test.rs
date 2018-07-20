use client;
use rocket::{
    self,
    http::{ContentType, Status},
};
use serde_json;

fn client() -> rocket::local::Client {
    rocket::local::Client::new(::rocket()).unwrap()
}

pub fn get_bus_stops() -> Vec<client::BusStopInfo> {
    let client = client();
    let mut response = client
        .get("/api/bus_stops?lat=47.653435&lon=-122.305641&lat_span=0.002&lon_span=0.003")
        .dispatch();
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.content_type(), Some(ContentType::JSON));
    let body = response.body_string().expect("body was empty");
    serde_json::from_str(body.as_str())
        .expect("Could not parse api response into 'Vec<client::BusStop>'")
}

pub fn get_bus_stop_status() -> client::BusStopStatus {
    let client = client();
    let mut response = client.get("/api/bus_stop_status/1_75403").dispatch();
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.content_type(), Some(ContentType::JSON));
    let body = response.body_string().expect("body was empty");
    serde_json::from_str(body.as_str())
        .expect("Could not parse api response into 'client::BusStopStatus'")
}

pub fn get_crime() -> String {
    let client = client();
    let mut response = client
        .get("/api/info?latitude=-122.33&longitude=47.59")
        .dispatch();
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.content_type(), Some(ContentType::JSON));
    response.body_string().expect("body was empty")
}

#[cfg(not(feature = "contract"))]
mod unit {
    use super::*;

    use mocktopus::mocking::{MockResult, Mockable};

    #[test]
    fn crime() {
        let mut location = None;
        unsafe {
            client::Client::info.mock_raw(|_, loc| {
                location = Some(loc);
                MockResult::Return(Ok("Hello".to_string()))
            });
        }

        get_crime();
    }

    #[test]
    fn bus_stops() {
        let mut area = None;
        let expected = vec![client::BusStopInfo {
            direction: String::from("S"),
            id: String::from("1_2345"),
            name: String::from("hello darkness"),
            lat: 1.23,
            lon: 123.23,
        }];
        unsafe {
            client::Client::bus_stops.mock_raw(|_, a| {
                area = Some(a);
                MockResult::Return(Ok(expected.clone()))
            });
        }

        let actual = get_bus_stops();
        assert_eq!(actual, expected);

        let area = area.expect("Client::bus_stops not called");
        assert_eq!(area.lat, 47.653435);
        assert_eq!(area.lon, -122.305641);
        assert_eq!(area.lat_span, 0.002);
        assert_eq!(area.lon_span, 0.003);
    }

    #[test]
    fn status() {
        let mut actual_stop = None;
        let expected_status = client::BusStopStatus {
            info: client::BusStopInfo {
                direction: String::from("S"),
                id: String::from("1_75403"),
                name: String::from("hello darkness"),
                lat: 1.23,
                lon: 123.23,
            },
            buses: vec![client::BusStatus {
                route: String::from("36E"),
                headsign: String::from("MAGNOLIA & FOO"),
                predicted_time: 1234093393,
                scheduled_time: 24203223,
            }],
        };
        unsafe {
            client::Client::bus_stop_status.mock_raw(|_, s| {
                actual_stop = Some(s.clone());
                MockResult::Return(Ok(expected_status.clone()))
            })
        }

        let actual_status = get_bus_stop_status();
        assert_eq!(actual_status, expected_status);

        let actual_stop = actual_stop.expect("'Client::bus_stop_status' not called");
        assert_eq!(actual_stop, "1_75403");
    }
}

#[cfg(feature = "contract")]
mod integration {
    use super::*;

    #[test]
    fn crime() {
        get_crime();
    }

    #[test]
    fn bus_stops() {
        let stops = get_bus_stops();
        assert!(stops.len() > 0);
    }

    #[test]
    fn status() {
        let status = get_bus_stop_status();
        assert!(status.buses.len() > 0);
    }
}
