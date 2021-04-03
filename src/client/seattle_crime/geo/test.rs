use super::*;

use approx::assert_relative_eq;
use mockito::mock;

#[test]
fn stops() {
    let location = Location {
        lon: -122.305641,
        lat: 47.653435,
    };
    let query_path = serde_urlencoded::to_string(&[
        (
            "geometry",
            format!("{lon},{lat}", lon = location.lon, lat = location.lat).as_str(),
        ),
        ("geometryType", "esriGeometryPoint"),
        ("inSR", "4326"),
        ("returnGeometry", "true"),
        ("f", "geojson"),
    ])
    .expect("could not encode query");
    let subject = Client::new(reqwest::blocking::Client::new(), mockito::server_url());
    let path = format!(
        "/ArcGIS/rest/services/DoIT_ext/SP_Precincts_Beats/MapServer/2/query?{}",
        query_path
    );

    let mock = mock("GET", path.as_str())
        .with_status(200)
        .with_body(include_str!("fixtures/precinct_beats.json"))
        .with_header("Content-Type", "text/plain")
        .create();

    let beat = subject
        .beat_for(location)
        .expect("expected a succesful Beat");

    mock.assert();
    assert_eq!(beat.name, String::from("U3"));
    assert_relative_eq!(beat.area_km, 16.62, max_relative = 0.003);
}
