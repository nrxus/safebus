use super::*;

use mockito::{mock, SERVER_URL};
use serde_urlencoded;

#[test]
fn stops() {
    let subject = Client::new(
        reqwest::Client::new(),
        String::from(SERVER_URL),
        String::from("SOME_KEY"),
    );
    let query = StopsQuery {
        lat: 32.3,
        lon: 23.1,
        lat_span: 0.01,
        lon_span: 0.0002,
        max_count: 9,
    };
    let query_path =
        serde_urlencoded::to_string(query.clone()).expect("could not encode 'StopsQuery'");
    let path = format!(
        "/api/where/stops-for-location.json?{}&key=SOME_KEY",
        query_path
    );

    let mock = mock("GET", path.as_str())
        .with_status(200)
        .with_body(include_str!("../fixtures/stop_list.json"))
        .with_header("Content-Type", "application/json")
        .create();

    let actual = subject
        .stops(&query)
        .expect("expected a succesful stops response");
    mock.assert();
    assert_eq!(
        actual,
        vec![
            StopInfo {
                id: String::from("1_75403"),
                direction: String::from("S"),
                name: String::from("Stevens Way & Benton Ln"),
                lat: 47.654365,
                lon: -122.305214,
            },
            StopInfo {
                id: String::from("1_75414"),
                direction: String::from("N"),
                name: String::from("Stevens Way & Benton Ln"),
                lat: 47.653713,
                lon: -122.305023,
            },
        ]
    )
}

#[test]
fn departures() {
    let subject = Client::new(
        reqwest::Client::new(),
        String::from(SERVER_URL),
        String::from("SOME_KEY"),
    );

    let path = "/api/where/arrivals-and-departures-for-stop/1_75403.json?key=SOME_KEY";

    let mock = mock("GET", path)
        .with_status(200)
        .with_body(include_str!(
            "../fixtures/arrivals-and-departures-for-stop.json"
        ))
        .with_header("Content-Type", "application/json")
        .create();

    let actual = subject
        .departures("1_75403")
        .expect("expected a succesful departures response");
    mock.assert();
    assert_eq!(
        actual,
        Departures {
            buses: vec![
                Status {
                    headsign: String::from("Northgate Roosevelt"),
                    route: String::from("67"),
                    predicted_time: 0,
                    scheduled_time: 1532043240000,
                },
                Status {
                    headsign: String::from("Central Magnolia Fremont"),
                    route: String::from("31"),
                    predicted_time: 1532043461000,
                    scheduled_time: 1532042940000,
                },
            ],
            stop: StopInfo {
                id: String::from("1_75403"),
                name: String::from("Stevens Way & Benton Ln"),
                direction: String::from("S"),
                lat: 47.654365,
                lon: -122.305214,
            },
        }
    );
}
