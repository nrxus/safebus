use api::Location;
use client::CrimeClient;
use query::Query;

use chrono::{Duration, Local};

pub struct Service<'c> {
    pub client: &'c CrimeClient,
}

impl<'c> Service<'c> {
    pub fn info(&self, location: Location) -> Result<String, String> {
        let start_date = Local::now() - Duration::days(180);
        let query = Query::new(location).and(start_date);
        self.client.request(query)
    }
}

#[cfg(all(test, not(feature = "contract")))]
mod test {
    use super::*;
    use mockito::mock;
    use providers;
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
        let client = providers::crime_client();
        let subject = Service { client: &client };
        let actual = subject.info(location);

        mock.assert();
        assert_eq!(actual, Ok("Hello".to_string()));
    }
}
