mod query;

pub use self::query::{Filter, Query};

use reqwest;
use std::collections::HashMap;

header! { (XAppToken, "X-App-Token") => [String]}

pub struct Client {
    host: String,
    token: XAppToken,
    http_client: reqwest::Client,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Crime {
    pub description: String,
    pub count: u32,
}

impl Client {
    pub fn new(http_client: reqwest::Client, host: String, token: String) -> Self {
        Client {
            host,
            http_client,
            token: XAppToken(token),
        }
    }
}

// allow users of Client to mock the requests in unit tests
#[cfg(all(test, not(feature = "contract")))]
use mocktopus::macros::mockable;

#[cfg_attr(all(test, not(feature = "contract")), mockable)]
impl Client {
    pub fn crimes(&self, query: &Query) -> Result<Vec<Crime>, String> {
        let url = format!("{}/{}", self.host, "resource/aj7i-nahf.json");
        self.http_client
            .get(&url)
            .header(self.token.clone())
            .query(query)
            .send()
            .and_then(|mut r| r.json())
            .map(into_crime)
            .map_err(|e| format!("{}", e))
    }
}

fn into_crime(responses: Vec<CrimeResponse>) -> Vec<Crime> {
    let mut hash = HashMap::new();
    for crime in responses.into_iter() {
        let count = hash.entry(crime.description).or_insert(0);
        *count += 1;
    }
    hash.into_iter()
        .map(|(description, count)| Crime { description, count })
        .collect()
}

#[derive(Deserialize)]
struct CrimeResponse {
    #[serde(rename = "crime_description")]
    description: String,
}

#[cfg(all(test, not(feature = "contract")))]
mod test;
