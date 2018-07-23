use super::*;

use chrono::Local;
use mockito::{mock, SERVER_URL};
use serde_urlencoded;

#[test]
fn crime() {
    let subject = Client::new(
        reqwest::Client::new(),
        String::from(SERVER_URL),
        String::from("SOME_TOKEN"),
    );

    let query = Query::new(Local::now());
    let query_path = serde_urlencoded::to_string(query.clone()).unwrap();
    let path = format!("/resource/policereport.json?{}", query_path);
    let mock = mock("GET", path.as_str())
        .with_status(200)
        .with_body("{}")
        .with_header("Content-Type", "application/json")
        .create();
    let actual = subject.crime(&query);
    mock.assert();

    assert_eq!(actual, Ok("{}".to_string()));
}
