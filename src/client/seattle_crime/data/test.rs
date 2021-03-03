use super::*;

use chrono::Local;
use mockito::mock;

#[test]
fn crime() {
    use super::query::Filter::*;

    let subject = Client::new(
        reqwest::blocking::Client::new(),
        mockito::server_url(),
        String::from("SOME_TOKEN"),
    );

    let query = Query::new(After(Local::now()));
    let query_path = serde_urlencoded::to_string(query.clone()).unwrap();
    let path = format!("/resource/tazs-3rd5.json?{}", query_path);
    let mock = mock("GET", path.as_str())
        .match_header("X-App-Token", "SOME_TOKEN")
        .with_status(200)
        .with_body(include_str!("fixtures/crime_data.json"))
        .with_header("Content-Type", "application/json")
        .create();

    let actual = subject.crimes(&query);
    mock.assert();

    let actual = actual.expect("expected succesful crime data");

    let expected = vec![
        Crime {
            description: String::from("ARSON"),
            count: 2,
        },
        Crime {
            description: String::from("ROBBERY"),
            count: 1,
        },
        Crime {
            description: String::from("ASSAULT OFFENSES"),
            count: 1,
        },
    ];
    assert!(actual.contains(&expected[0]));
    assert!(actual.contains(&expected[1]));
    assert!(actual.contains(&expected[2]));
    assert_eq!(actual.len(), 3);
}
