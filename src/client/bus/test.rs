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
            Stop {
                id: String::from("1_75403"),
                direction: String::from("S"),
                name: String::from("Stevens Way & Benton Ln"),
                lat: 47.654365,
                lon: -122.305214,
            },
            Stop {
                id: String::from("1_75414"),
                direction: String::from("N"),
                name: String::from("Stevens Way & Benton Ln"),
                lat: 47.653713,
                lon: -122.305023,
            },
        ]
    )
}
