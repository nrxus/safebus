use reqwest;

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct StopsQuery {
    pub lat: f64,
    pub lon: f64,
    pub lat_span: f64,
    pub lon_span: f64,
    pub max_count: i8,
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

impl Client {
    pub fn new(http_client: reqwest::Client, host: String, key: String) -> Self {
        Client {
            host,
            http_client,
            key,
        }
    }
}

// allow users of Client to mock the requests in unit tests
#[cfg(all(test, not(feature = "contract")))]
use mocktopus::macros::mockable;

#[cfg_attr(all(test, not(feature = "contract")), mockable)]
impl Client {
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

#[derive(Debug, Deserialize)]
struct StopsListResponse {
    data: StopsListData,
}

#[derive(Debug, Deserialize)]
struct StopsListData {
    list: Vec<Stop>,
}

#[cfg(all(test, not(feature = "contract")))]
mod test;
