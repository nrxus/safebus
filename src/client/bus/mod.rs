use reqwest;

#[cfg(all(test, not(feature = "contract")))]
use mocktopus::macros::mockable;

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct StopsQuery {
    pub lat: f64,
    pub lon: f64,
    pub lat_span: f64,
    pub lon_span: f64,
    pub max_count: i8,
}

#[derive(Debug, Deserialize)]
struct StopsListResponse {
    data: StopsListData,
}

#[derive(Debug, Deserialize)]
struct StopsListData {
    list: Vec<Stop>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Stop {
    pub id: String,
    pub direction: String,
    pub lat: f64,
    pub lon: f64,
    pub name: String,
}

pub struct Client {
    host: String,
    key: String,
    http_client: reqwest::Client,
}

#[cfg_attr(all(test, not(feature = "contract")), mockable)]
impl Client {
    pub fn new(http_client: reqwest::Client, host: String, key: String) -> Self {
        Client {
            host,
            http_client,
            key,
        }
    }

    pub fn stops(&self, query: &StopsQuery) -> Result<Vec<Stop>, String> {
        let url = format!("{}/{}", self.host, "api/where/stops-for-location.json");
        self.http_client
            .get(&url)
            .query(query)
            .query(&[("key", self.key.as_str())])
            .send()
            .and_then(|mut r| r.json())
            .map(|r: StopsListResponse| r.data.list)
            .map_err(|e| format!("{}", e))
    }
}

#[cfg(all(test, not(feature = "contract")))]
mod test {
    use super::*;

    use mockito::{mock, SERVER_URL};
    use serde_urlencoded;

    #[test]
    fn stops() {
        let subject = Client::new(
            reqwest::Client::new(),
            String::from(SERVER_URL),
            String::from("SOME_KEY"),
        );
        let query = StopsQuery {
            lat: 32.3,
            lon: 23.1,
            lat_span: 0.01,
            lon_span: 0.0002,
            max_count: 9,
        };
        let query_path =
            serde_urlencoded::to_string(query.clone()).expect("could not encode 'StopsQuery'");
        let path = format!(
            "/api/where/stops-for-location.json?{}&key=SOME_KEY",
            query_path
        );

        let mock = mock("GET", path.as_str())
            .with_status(200)
            .with_body(include_str!("../fixtures/stop_list.json"))
            .with_header("Content-Type", "application/json")
            .create();

        let actual = subject
            .stops(&query)
            .expect("expected a succesful stops response");
        mock.assert();
        assert_eq!(
            actual,
            vec![
                Stop {
                    id: String::from("1_75403"),
                    direction: String::from("S"),
                    name: String::from("Stevens Way & Benton Ln"),
                    lat: 47.654365,
                    lon: -122.305214,
                },
                Stop {
                    id: String::from("1_75414"),
                    direction: String::from("N"),
                    name: String::from("Stevens Way & Benton Ln"),
                    lat: 47.653713,
                    lon: -122.305023,
                },
            ]
        )
    }
}
