use super::*;

use faux::when;

#[test]
fn bus_stops() {
    let mut bus_client = bus::Client::faux();

    let expected_stops = vec![bus::StopInfo {
        id: String::from("1_1234"),
        direction: String::from("N"),
        name: String::from("some bus"),
        lat: 34.3199,
        lon: 23.12005,
    }];

    unsafe {
        when!(bus_client.stops).then(|q| {
            assert_eq!(
                *q,
                bus::StopsQuery {
                    lat: 34.32,
                    lon: 23.12,
                    lat_span: 0.002,
                    lon_span: 0.0005,
                    max_count: 20,
                }
            );
            Ok(expected_stops.clone())
        })
    }

    let area = Area {
        lat: 34.32,
        lon: 23.12,
        lat_span: 0.002,
        lon_span: 0.0005,
        limit: None,
    };

    let subject = Client::new(seattle_crime::Service::faux(), bus_client);

    let actual_stops = subject
        .bus_stops(&area)
        .expect("expected a succesful bus stop response");

    assert_eq!(actual_stops, expected_stops);
}

#[test]
fn bus_stops_with_limit() {
    let mut bus_client = bus::Client::faux();

    unsafe {
        when!(bus_client.stops).then(|q| {
            assert_eq!(q.max_count, 56);
            Ok(vec![])
        });
    }

    let area = Area {
        lat: 34.32,
        lon: 23.12,
        lat_span: 0.002,
        lon_span: 0.0005,
        limit: Some(56),
    };

    let subject = Client::new(seattle_crime::Service::faux(), bus_client);

    subject
        .bus_stops(&area)
        .expect("expected a succesful bus stop response");
}

#[test]
fn bus_stop_status() {
    let mut crime_service = seattle_crime::Service::faux();
    let mut bus_client = bus::Client::faux();

    let departure_info = bus::Departures {
        buses: vec![bus::Status {
            headsign: String::from("MAGNOLIA PKWY"),
            route: String::from("26E"),
            predicted_time: 222324334,
            scheduled_time: 232343432,
        }],
        stop: bus::StopInfo {
            direction: String::from("N"),
            id: String::from("1_2345"),
            name: String::from("some name"),
            lat: 233.232233,
            lon: -123.23322,
        },
    };
    let related_crimes = vec![Crime {
        description: String::from("ROBBERY"),
        density: 0.23,
    }];
    let unrelated_crimes = vec![Crime {
        description: String::from("ROBBERY"),
        density: 0.23,
    }];
    unsafe {
        when!(bus_client.departures).then(|b| {
            assert_eq!(b, "3_232");
            Ok(departure_info.clone())
        });
    }
    unsafe {
        when!(crime_service.crime_nearby).then(|actual_location| {
            let expected_location = seattle_crime::Location {
                lat: departure_info.stop.lat,
                lon: departure_info.stop.lon,
            };
            assert_eq!(actual_location, expected_location);
            Ok(seattle_crime::CrimeData {
                related_crimes: related_crimes.clone(),
                unrelated_crimes: unrelated_crimes.clone(),
            })
        });
    }

    let subject = Client::new(crime_service, bus_client);
    
    let actual_status = subject
        .bus_stop_status("3_232")
        .expect("expected a succesful bus stop status");
    let expected_status = BusStopStatus {
        buses: departure_info.buses,
        info: departure_info.stop,
        related_crimes,
        unrelated_crimes,
    };
    assert_eq!(actual_status, expected_status);
}
