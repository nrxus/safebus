use super::*;

use chrono::{Duration, Local};
use faux::when;

#[test]
fn crime_nearby() {
    let mut data_client = data::Client::faux();
    let mut geo_client = geo::Client::faux();
    let beat = geo::Beat {
        name: String::from("U2"),
        area_km: 13.2,
    };
    let location = Location {
        lat: 23.32,
        lon: 32.22,
    };

    when!(geo_client.beat_for(location)).then_return(Ok(beat.clone()));

    let crimes = vec![
        data::Crime {
            description: String::from("ROBBERY"),
            count: 20,
        },
        data::Crime {
            description: String::from("FRAUD OFFENSES"),
            count: 12,
        },
        data::Crime {
            description: String::from("WEAPON LAW VIOLATIONS"),
            count: 15,
        },
    ];

    let three_months_ago = Local::now() - Duration::days(90);
    let expected_query = data::Query::new(After(three_months_ago)).and(Beat(beat.name.clone()));
    when!(data_client.crimes(expected_query)).then_return(Ok(crimes.clone()));

    let subject = Service::new(data_client, geo_client);
    let actual = subject
        .crime_nearby(location)
        .expect("expected succesful crime data");
    let crimes: Vec<_> = crimes
        .iter()
        .map(|c| Crime {
            description: c.description.clone(),
            density: f64::from(c.count) / beat.area_km,
        })
        .collect();
    let related_crimes = vec![crimes[0].clone(), crimes[2].clone()];
    let unrelated_crimes = vec![crimes[1].clone()];
    let expected = CrimeData {
        related_crimes,
        unrelated_crimes,
    };
    assert_eq!(actual, expected);
}
