mod bus;
mod seattle_data;

pub use self::bus::StopInfo as BusStopInfo;
use api::{Area, Location};

use chrono::{Duration, Local};
use reqwest;

pub struct Client {
    seattle_client: seattle_data::Client,
    bus_client: bus::Client,
}

// in non-unit test get the secrets from the environment variables
#[cfg(any(not(test), feature = "contract"))]
impl Client {
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
}

// allow users of Client to mock the requests in unit tests
#[cfg(all(test, not(feature = "contract")))]
use mocktopus::macros::mockable;

#[cfg_attr(all(test, not(feature = "contract")), mockable)]
impl Client {
    pub fn info(&self, location: Location) -> Result<String, String> {
        let start_date = Local::now() - Duration::days(180);
        let query = seattle_data::Query::new(location).and(start_date);
        self.seattle_client.crime(&query)
    }

    pub fn bus_stops(&self, area: Area) -> Result<Vec<bus::StopInfo>, String> {
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
mod test;
