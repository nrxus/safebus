use crate::client;
use rocket::{
    self,
    http::{ContentType, Status},
};

fn rocket_client(safebus_client: client::Client) -> rocket::local::Client {
    rocket::local::Client::new(crate::rocket(safebus_client)).unwrap()
}

pub fn get_bus_stops_limited(safebus_client: client::Client) -> Vec<client::BusStopInfo> {
    let client = rocket_client(safebus_client);
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

pub fn get_bus_stop_status(safebus_client: client::Client) -> client::BusStopStatus {
    let client = rocket_client(safebus_client);
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
    use crate::api::Area;

    use super::*;
    use faux::when;

    #[test]
    fn bus_stops_with_limit() {
        let expected = vec![client::BusStopInfo {
            direction: String::from("S"),
            id: String::from("1_2345"),
            name: String::from("hello darkness"),
            lat: 1.23,
            lon: 123.23,
        }];

        let mut client = client::Client::faux();
        when!(client.bus_stops(Area {
            lat: 47.653435,
            lon: -122.305641,
            lat_span: 0.002,
            lon_span: 0.003,
            limit: Some(100),
        }))
        .then_return(Ok(expected.clone()));

        let actual = get_bus_stops_limited(client);
        assert_eq!(actual, expected);
    }

    #[test]
    fn bus_stops_with_no_limit() {
        let mut client = client::Client::faux();
        when!(client.bus_stops(*_ = faux::pattern!(Area { limit: None, .. })))
            .then_return(Ok(vec![]));

        let client = rocket_client(client);
        let response = client
            .get("/api/bus_stops?lat=47.653435&lon=-122.305641&lat_span=0.002&lon_span=0.003")
            .dispatch();

        assert_eq!(response.status(), Status::Ok);
    }

    #[test]
    fn status() {
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
        let mut client = client::Client::faux();
        when!(client.bus_stop_status("1_75403")).then_return(Ok(expected_status.clone()));

        let actual_status = get_bus_stop_status(client);
        assert_eq!(actual_status, expected_status);
    }
}

#[cfg(feature = "integration")]
mod integration {
    use super::*;

    #[test]
    fn bus_stops() {
        let stops = get_bus_stops_limited(client::Client::from_http_client(
            reqwest::blocking::Client::new(),
        ));
        assert!(stops.len() > 0);
    }

    #[test]
    fn status() {
        let status = get_bus_stop_status(client::Client::from_http_client(
            reqwest::blocking::Client::new(),
        ));
        assert!(status.buses.len() > 0);
        assert!(status.related_crimes.len() > 0);
        assert!(status.unrelated_crimes.len() > 0);
    }
}
