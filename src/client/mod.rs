mod bus;
mod seattle_data;

pub use self::bus::Stop as BusStop;
use api::{Area, Location};

use chrono::{Duration, Local};
use reqwest;

#[cfg(all(test, not(feature = "contract")))]
use mocktopus::macros::mockable;

pub struct Client {
    seattle_client: seattle_data::Client,
    bus_client: bus::Client,
}

impl Client {
    #[cfg(any(not(test), feature = "contract"))]
    pub fn new(http_client: reqwest::Client) -> Self {
        fn expect_env(name: &str) -> String {
            use std::env;
            env::var(name).expect(&format!("'{}' ENV VARIABLE IS REQUIRED", name))
        }

        let seattle_client = {
            let token = expect_env("SEATTLE_API_KEY");
            let host = "https://data.seattle.gov/".to_string();
            seattle_data::Client::new(http_client.clone(), host, token)
        };
        let bus_client = {
            let key = expect_env("ONEBUSAWAY_API_KEY");
            let host = "http://api.pugetsound.onebusaway.org/".to_string();
            bus::Client::new(http_client, host, key)
        };
        Client {
            seattle_client,
            bus_client,
        }
    }

    #[cfg(all(test, not(feature = "contract")))]
    pub fn new(http_client: reqwest::Client) -> Self {
        let seattle_client = seattle_data::Client::new(
            http_client.clone(),
            String::from("http://localhost"),
            String::from("SEATTLE_TOKEN"),
        );
        let bus_client = bus::Client::new(
            http_client,
            String::from("http://localhost"),
            String::from("BUS_KEY"),
        );

        Client {
            seattle_client,
            bus_client,
        }
    }
}

#[cfg_attr(all(test, not(feature = "contract")), mockable)]
impl Client {
    pub fn info(&self, location: Location) -> Result<String, String> {
        let start_date = Local::now() - Duration::days(180);
        let query = seattle_data::Query::new(location).and(start_date);
        self.seattle_client.request(&query)
    }

    pub fn bus_stops(&self, area: Area) -> Result<Vec<bus::Stop>, String> {
        self.bus_client.stops(&bus::StopsQuery {
            lat: area.lat,
            lon: area.lon,
            lat_span: area.lat_span,
            lon_span: area.lon_span,
            max_count: 20,
        })
    }
}

#[cfg(all(test, not(feature = "contract")))]
mod test {
    use super::*;

    use mocktopus::mocking::{MockResult, Mockable};

    #[test]
    fn info() {
        let subject = Client::new(reqwest::Client::new());
        let expected = String::from("{}");

        let mut query = None;
        unsafe {
            seattle_data::Client::request.mock_raw(|_, q| {
                query = Some(q.clone());
                MockResult::Return(Ok(expected.clone()))
            })
        }

        let location = Location {
            latitude: 32.2,
            longitude: 67.23,
        };
        let actual = subject
            .info(location)
            .expect("expected a succesful crime response");

        assert_eq!(actual, expected);
        let query = query.expect("seattle_data::Client::request not called");
        assert_eq!(
            query,
            seattle_data::Query::new(location).and(Local::now() - Duration::days(180))
        );
    }

    #[test]
    fn bus_stops() {
        let subject = Client::new(reqwest::Client::new());
        let mut query = None;
        let expected_stops = vec![bus::Stop {
            id: String::from("1_1234"),
            direction: String::from("N"),
            name: String::from("some bus"),
            lat: 34.3199,
            lon: 23.12005,
        }];
        unsafe {
            bus::Client::stops.mock_raw(|_, q| {
                query = Some(q.clone());
                MockResult::Return(Ok(expected_stops.clone()))
            });
        }

        let area = Area {
            lat: 34.32,
            lon: 23.12,
            lat_span: 0.002,
            lon_span: 0.0005,
        };
        let actual_stops = subject
            .bus_stops(area)
            .expect("expected a succesful bus stop response");
        assert_eq!(actual_stops, expected_stops);
        let query = query.expect("'bus::Client::stops' not called");
        assert_eq!(
            query,
            bus::StopsQuery {
                lat: 34.32,
                lon: 23.12,
                lat_span: 0.002,
                lon_span: 0.0005,
                max_count: 20,
            }
        )
    }
}
