mod bus;
mod seattle_crime;

pub use self::bus::Status as BusStatus;
pub use self::bus::StopInfo as BusStopInfo;
use api::Area;

use reqwest;

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct BusStopStatus {
    #[serde(flatten)]
    pub info: bus::StopInfo,
    pub buses: Vec<bus::Status>,
}

pub struct Client {
    crime_service: seattle_crime::Service,
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

        let crime_service = {
            let token = expect_env("SEATTLE_API_KEY");
            let host = "https://data.seattle.gov/".to_string();
            let data_client = seattle_crime::data::Client::new(http_client.clone(), host, token);
            seattle_crime::Service::new(data_client)
        };
        let bus_client = {
            let key = expect_env("ONEBUSAWAY_API_KEY");
            let host = "http://api.pugetsound.onebusaway.org/".to_string();
            bus::Client::new(http_client, host, key)
        };
        Client {
            crime_service,
            bus_client,
        }
    }
}

// allow users of Client to mock the requests in unit tests
#[cfg(all(test, not(feature = "contract")))]
use mocktopus::macros::mockable;

#[cfg_attr(all(test, not(feature = "contract")), mockable)]
impl Client {
    pub fn bus_stops(&self, area: Area) -> Result<Vec<bus::StopInfo>, String> {
        self.bus_client.stops(&bus::StopsQuery {
            lat: area.lat,
            lon: area.lon,
            lat_span: area.lat_span,
            lon_span: area.lon_span,
            max_count: 20,
        })
    }

    pub fn bus_stop_status(&self, stop_id: &str) -> Result<BusStopStatus, String> {
        self.bus_client.departures(stop_id).map(|b| BusStopStatus {
            info: b.stop,
            buses: b.buses,
        })
    }
}

#[cfg(all(test, not(feature = "contract")))]
mod test;
