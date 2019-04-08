mod query;

pub use self::query::{Filter, Query};

use std::collections::HashMap;

pub struct Client {
    host: String,
    token: String,
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
            token,
        }
    }
}

// allow users of Client to mock the requests in unit tests
#[cfg(all(test, not(feature = "integration")))]
use mocktopus::macros::mockable;

#[cfg_attr(all(test, not(feature = "integration")), mockable)]
impl Client {
    pub fn crimes(&self, query: &Query) -> Result<Vec<Crime>, String> {
        let url = format!("{}/{}", self.host, "resource/xurz-654a.json");
        self.http_client
            .get(&url)
            .header("X-App-Token", self.token.as_str())
            .query(query)
            .send()
            .and_then(reqwest::Response::error_for_status)
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

#[derive(serde::Deserialize)]
struct CrimeResponse {
    #[serde(rename = "crime_description")]
    description: String,
}

#[cfg(all(test, not(feature = "integration")))]
mod test;
