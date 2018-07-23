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
}

// allow users of Client to mock the requests in unit tests
#[cfg(all(test, not(feature = "contract")))]
use mocktopus::macros::mockable;

#[cfg_attr(all(test, not(feature = "contract")), mockable)]
impl Client {
    pub fn crime(&self, query: &Query) -> Result<String, String> {
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
mod test;
