mod crime_client;
mod providers;
mod query;

use self::{crime_client::CrimeClient, query::Query};
use api::Location;

use chrono::{Duration, Local};
use reqwest;

pub struct Client {
    crime_client: CrimeClient,
}

impl Client {
    pub fn new(http_client: reqwest::Client) -> Self {
        let crime_client = providers::crime_client(http_client);
        Client { crime_client }
    }

    pub fn info(&self, location: Location) -> Result<String, String> {
        let start_date = Local::now() - Duration::days(180);
        let query = Query::new(location).and(start_date);
        self.crime_client.request(query)
    }
}

#[cfg(all(test, not(feature = "contract")))]
mod test {
    use super::*;

    use mockito::mock;
    use url::percent_encoding::{utf8_percent_encode, DEFAULT_ENCODE_SET};

    #[test]
    fn info() {
        let location = Location {
            latitude: 32.2,
            longitude: 67.23,
        };
        let query = Query::new(location)
            .and(Local::now() - Duration::days(180))
            .to_string();
        let query = utf8_percent_encode(query.as_str(), DEFAULT_ENCODE_SET);
        let path = format!("/resource/policereport.json?{}", query);
        let mock = mock("GET", path.as_str()).with_body("Hello").create();
        let crime_client = providers::crime_client(reqwest::Client::new());
        let subject = Client {
            crime_client: crime_client,
        };
        let actual = subject.info(location);

        mock.assert();
        assert_eq!(actual, Ok("Hello".to_string()));
    }
}
