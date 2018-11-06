use crate::client;
use rocket::{
    self,
    http::{ContentType, Status},
};
use serde_json;

fn client() -> rocket::local::Client {
    rocket::local::Client::new(crate::rocket()).unwrap()
}

pub fn get_bus_stops_limited() -> Vec<client::BusStopInfo> {
    let client = client();
    let mut response = client
        .get("/api/bus_stops?lat=47.653435&lon=-122.305641&lat_span=0.002&lon_span=0.003&limit=100")
        .dispatch();
    if response.status() != Status::Ok {
        panic!(
            "expected Status::OK but got {:?} with body {:?}",
            response.status(),
            response.body_string()
        );
    }
    assert_eq!(response.content_type(), Some(ContentType::JSON));
    let body = response.body_string().expect("body was empty");
    serde_json::from_str(body.as_str())
        .expect("Could not parse api response into 'Vec<client::BusStop>'")
}

pub fn get_bus_stop_status() -> client::BusStopStatus {
    let client = client();
    let mut response = client.get("/api/bus_stop_status/1_75403").dispatch();
    if response.status() != Status::Ok {
        panic!(
            "expected Status::OK but got {:?} with body {:?}",
            response.status(),
            response.body_string()
        );
    }
    assert_eq!(response.content_type(), Some(ContentType::JSON));
    let body = response.body_string().expect("body was empty");
    serde_json::from_str(body.as_str())
        .expect("Could not parse api response into 'client::BusStopStatus'")
}

#[cfg(not(feature = "integration"))]
mod unit {
    use super::*;

    use mocktopus::mocking::{MockResult, Mockable};

    #[test]
    fn bus_stops_with_limit() {
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
                area = Some(*a);
                MockResult::Return(Ok(expected.clone()))
            });
        }

        let actual = get_bus_stops_limited();
        assert_eq!(actual, expected);

        let area = area.expect("Client::bus_stops not called");
        assert_eq!(area.lat, 47.653435);
        assert_eq!(area.lon, -122.305641);
        assert_eq!(area.lat_span, 0.002);
        assert_eq!(area.lon_span, 0.003);
        assert_eq!(area.limit, Some(100));
    }

    #[test]
    fn bus_stops_with_no_limit() {
        let mut called = false;
        unsafe {
            client::Client::bus_stops.mock_raw(|_, a| {
                called = true;
                assert_eq!(a.limit, None);
                MockResult::Return(Ok(vec![]))
            });
        }

        let client = client();
        let response = client
            .get("/api/bus_stops?lat=47.653435&lon=-122.305641&lat_span=0.002&lon_span=0.003")
            .dispatch();

        assert_eq!(response.status(), Status::Ok);
        assert!(called);
    }

    #[test]
    fn status() {
        let mut called = false;
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
            related_crimes: vec![client::Crime {
                description: String::from("ROBBERY"),
                density: 0.23,
            }],
            unrelated_crimes: vec![client::Crime {
                description: String::from("NARCOTICS"),
                density: 0.111,
            }],
        };
        unsafe {
            client::Client::bus_stop_status.mock_raw(|_, stop| {
                called = true;
                assert_eq!(stop, "1_75403");
                MockResult::Return(Ok(expected_status.clone()))
            })
        }

        let actual_status = get_bus_stop_status();
        assert_eq!(actual_status, expected_status);
        assert!(true, "'Client::bus_stop_status' not called");
    }
}

#[cfg(feature = "integration")]
mod integration {
    use super::*;

    #[test]
    fn bus_stops() {
        let stops = get_bus_stops_limited();
        assert!(stops.len() > 0);
    }

    #[test]
    fn status() {
        let status = get_bus_stop_status();
        assert!(status.buses.len() > 0);
        assert!(status.related_crimes.len() > 0);
        assert!(status.unrelated_crimes.len() > 0);
    }
}
