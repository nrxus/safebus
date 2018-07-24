mod query;

pub use self::query::{Filter, Query};

use reqwest;

header! { (XAppToken, "X-App-Token") => [String]}

pub struct Client {
    host: String,
    token: XAppToken,
    http_client: reqwest::Client,
}

#[derive(Clone)]
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
        Ok(vec![])
        // let url = format!("{}/{}", self.host, "resource/policereport.json");
        // self.http_client
        //     .get(&url)
        //     .header(self.token.clone())
        //     .query(query)
        //     .send()
        //     .and_then(|mut r| r.text())
        //     .map_err(|e| format!("{}", e))
    }
}

#[cfg(all(test, not(feature = "contract")))]
mod test;
