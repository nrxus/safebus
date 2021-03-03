mod query;

pub use self::query::{Filter, Query};

use std::collections::HashMap;

#[cfg_attr(test, faux::create)]
pub struct Client {
    host: String,
    token: String,
    http_client: reqwest::blocking::Client,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Crime {
    pub description: String,
    pub count: u32,
}

#[cfg_attr(test, faux::methods)]
impl Client {
    pub fn new(http_client: reqwest::blocking::Client, host: String, token: String) -> Self {
        Client {
            host,
            http_client,
            token,
        }
    }

    pub fn crimes(&self, query: &Query) -> Result<Vec<Crime>, String> {
        let url = format!("{}/{}", self.host, "resource/tazs-3rd5.json");
        self.http_client
            .get(&url)
            .header("X-App-Token", self.token.as_str())
            .query(query)
            .send()
            .and_then(reqwest::blocking::Response::error_for_status)
            .and_then(reqwest::blocking::Response::json)
            .map(into_crime)
            .map_err(|e| format!("{}", e))
    }
}

fn into_crime(responses: Vec<CrimeResponse>) -> Vec<Crime> {
    let mut hash = HashMap::new();
    for crime in responses.into_iter() {
        let count = hash.entry(crime.offense_parent_group).or_insert(0);
        *count += 1;
    }
    hash.into_iter()
        .map(|(description, count)| Crime { description, count })
        .collect()
}

#[derive(serde::Deserialize, Debug)]
struct CrimeResponse {
    offense_parent_group: String,
}

#[cfg(all(test, not(feature = "integration")))]
mod test;
