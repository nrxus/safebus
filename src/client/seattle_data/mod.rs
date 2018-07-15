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

    pub fn request(&self, query: Query) -> Result<String, String> {
        let url = format!("{}/{}?{}", self.host, "resource/policereport.json", query);
        self.http_client
            .get(&url)
            .header(self.token.clone())
            .send()
            .and_then(|mut r| r.text())
            .map_err(|e| format!("{}", e))
    }
}
