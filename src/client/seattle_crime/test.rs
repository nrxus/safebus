use super::*;

use chrono::{Duration, Local};
use mockito::SERVER_URL;
use mocktopus::mocking::{MockResult, Mockable};
use reqwest;

#[test]
fn crime_nearby() {
    let http_client = reqwest::Client::new();
    let host = String::from(SERVER_URL);
    let data_client = data::Client::new(
        http_client.clone(),
        host.clone(),
        String::from("SOME_TOKEN"),
    );
    let geo_client = geo::Client::new(http_client, host);
    let beat = geo::Beat {
        name: String::from("U2"),
        area_km: 13.2,
    };
    let location = Location {
        lat: 23.32,
        lon: 32.22,
    };

    {
        let beat = beat.clone();
        let location = location.clone();
        geo::Client::beat_for.mock_safe(move |_, l| {
            assert_eq!(location, l);
            MockResult::Return(Ok(beat.clone()))
        });
    }

    let crimes = vec![
        data::Crime {
            description: String::from("ROBBERY-STREET-GUN"),
            count: 20,
        },
        data::Crime {
            description: String::from("FORGERY-CHECK"),
            count: 12,
        },
        data::Crime {
            description: String::from("WEAPON-DISCHARGE"),
            count: 15,
        },
    ];

    {
        let crimes = crimes.clone();
        let beat = beat.name.clone();

        data::Client::crimes.mock_safe(move |_, q| {
            let six_months_ago = Local::now() - Duration::days(180);
            let expected_query = data::Query::new(After(six_months_ago)).and(Beat(beat.clone()));
            assert_eq!(&expected_query, q);
            MockResult::Return(Ok(crimes.clone()))
        });
    }

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
