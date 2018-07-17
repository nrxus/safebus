mod seattle_data;

use api::Location;

use std::env;

use chrono::{Duration, Local};
use reqwest;

#[cfg(all(test, not(feature = "contract")))]
use mocktopus::macros::mockable;

pub struct Client {
    seattle_client: seattle_data::Client,
}

#[cfg_attr(all(test, not(feature = "contract")), mockable)]
impl Client {
    pub fn new(http_client: reqwest::Client) -> Self {
        let token =
            env::var("SEATTLE_API_KEY").expect("'SEATTLE_API_KEY' ENV VARIABLE IS REQUIRED");
        let host = "https://data.seattle.gov/".to_string();
        let seattle_client = seattle_data::Client::new(http_client, host, token);
        Client { seattle_client }
    }

    pub fn info(&self, location: Location) -> Result<String, String> {
        let start_date = Local::now() - Duration::days(180);
        let query = seattle_data::Query::new(location).and(start_date);
        self.seattle_client.request(&query)
    }
}

#[cfg(all(test, not(feature = "contract")))]
mod test {
    use super::*;

    use mocktopus::mocking::{MockResult, Mockable};

    #[test]
    fn info() {
        let mut query = None;
        unsafe {
            seattle_data::Client::request.mock_raw(|_, q| {
                query = Some(q);
                MockResult::Return(Ok("{}".to_string()))
            })
        }
        let subject = Client::new(reqwest::Client::new());

        let location = Location {
            latitude: 32.2,
            longitude: 67.23,
        };
        let actual = subject.info(location);

        assert_eq!(actual, Ok("{}".to_string()));
    }
}
