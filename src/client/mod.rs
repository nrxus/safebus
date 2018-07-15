mod providers;
mod seattle_data;

use api::Location;

use chrono::{Duration, Local};
use reqwest;

pub struct Client {
    seattle_client: seattle_data::Client,
}

impl Client {
    pub fn new(http_client: reqwest::Client) -> Self {
        let seattle_client = providers::seattle_client(http_client);
        Client { seattle_client }
    }

    pub fn info(&self, location: Location) -> Result<String, String> {
        let start_date = Local::now() - Duration::days(180);
        let query = seattle_data::Query::new(location).and(start_date);
        self.seattle_client.request(query)
    }
}

#[cfg(all(test, not(feature = "contract")))]
mod test {
    use super::*;

    use mockito::mock;
    use url::percent_encoding::{utf8_percent_encode, DEFAULT_ENCODE_SET};

    #[test]
    fn info() {
        let seattle_client = providers::seattle_client(reqwest::Client::new());
        let subject = Client { seattle_client };

        let location = Location {
            latitude: 32.2,
            longitude: 67.23,
        };
        let query = seattle_data::Query::new(location)
            .and(Local::now() - Duration::days(180))
            .to_string();
        let query = utf8_percent_encode(query.as_str(), DEFAULT_ENCODE_SET);
        let path = format!("/resource/policereport.json?{}", query);
        let mock = mock("GET", path.as_str()).with_body("Hello").create();
        let actual = subject.info(location);

        mock.assert();
        assert_eq!(actual, Ok("Hello".to_string()));
    }
}
