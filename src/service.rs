use api::Location;
use client::CrimeClient;
use query::Query;

use chrono::{Duration, Local};

pub fn info(location: Location, client: &CrimeClient) -> Result<String, String> {
    let start_date = Local::now() - Duration::days(180);
    let query = Query::new(location).and(start_date);
    client.request(query)
}

#[cfg(all(test, not(feature = "contract")))]
mod test {
    use super::*;
    use {client, service};

    #[test]
    fn info() {
        let location = Location {
            latitude: 32.2,
            longitude: 67.23,
        };
        let expected = Ok("Hello".into());
        let mut client = client::CrimeClient::new();
        client.set_response(expected.clone());
        let actual = service::info(location, &client);
        assert_eq!(actual, expected);
        let requests = client.requests();
        assert_eq!(requests.len(), 1);
        assert_eq!(
            requests[0],
            Query::new(location).and(Local::now() - Duration::days(180))
        );
    }
}
