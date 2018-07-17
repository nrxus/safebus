mod query;

pub use self::query::Query;

use reqwest;

header! { (XAppToken, "X-App-Token") => [String]}

pub struct Client {
    host: String,
    token: XAppToken,
    http_client: reqwest::Client,
}

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
pub mod mock_data {
    use mockito::{mock, Matcher, Mock};

    pub fn happy_crime() -> Mock {
        let mock = mock("GET", Matcher::Regex(r"^/seattle_client.*$".to_string()));
        set_json_ok(mock).create()
    }

    #[cfg(all(test, not(feature = "contract")))]
    pub fn set_json_ok(mock: Mock) -> Mock {
        mock.with_status(200)
            .with_body("{}")
            .with_header("Content-Type", "application/json")
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
        let host = format!("{}/seattle_client", SERVER_URL);
        let subject = Client::new(reqwest::Client::new(), host, "SOME_TOKEN".to_string());

        let location = Location {
            latitude: 32.2,
            longitude: 67.23,
        };
        let query = Query::new(location);
        let query_path = serde_urlencoded::to_string(query.clone()).unwrap();
        let path = format!("/seattle_client/resource/policereport.json?{}", query_path);
        let mock = mock("GET", path.as_str());
        let mock = mock_data::set_json_ok(mock).create();
        let actual = subject.request(&query);
        mock.assert();

        assert_eq!(actual, Ok("{}".to_string()));
    }
}
