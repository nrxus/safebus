mod query;

pub use self::query::Query;

use reqwest;

#[cfg(all(test, not(feature = "contract")))]
use mocktopus::macros::mockable;

header! { (XAppToken, "X-App-Token") => [String]}

pub struct Client {
    host: String,
    token: XAppToken,
    http_client: reqwest::Client,
}

#[cfg_attr(all(test, not(feature = "contract")), mockable)]
impl Client {
    pub fn new(http_client: reqwest::Client, host: String, token: String) -> Self {
        Client {
            host,
            http_client,
            token: XAppToken(token),
        }
    }

    pub fn request(&self, query: &Query) -> Result<String, String> {
        let url = format!("{}/{}", self.host, "resource/policereport.json");
        self.http_client
            .get(&url)
            .header(self.token.clone())
            .query(query)
            .send()
            .and_then(|mut r| r.text())
            .map_err(|e| format!("{}", e))
    }
}

#[cfg(all(test, not(feature = "contract")))]
mod test {
    use super::*;
    use api::Location;

    use mockito::{mock, SERVER_URL};
    use serde_urlencoded;

    #[test]
    fn request() {
        let subject = Client::new(
            reqwest::Client::new(),
            String::from(SERVER_URL),
            String::from("SOME_TOKEN"),
        );

        let location = Location {
            latitude: 32.2,
            longitude: 67.23,
        };
        let query = Query::new(location);
        let query_path = serde_urlencoded::to_string(query.clone()).unwrap();
        let path = format!("/resource/policereport.json?{}", query_path);
        let mock = mock("GET", path.as_str())
            .with_status(200)
            .with_body("{}")
            .with_header("Content-Type", "application/json")
            .create();
        let actual = subject.request(&query);
        mock.assert();

        assert_eq!(actual, Ok("{}".to_string()));
    }
}
