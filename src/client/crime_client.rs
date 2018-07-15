use client::query::Query;
use reqwest;

header! { (XAppToken, "X-App-Token") => [String]}

pub struct CrimeClient {
    host: String,
    token: XAppToken,
    client: reqwest::Client,
}

impl CrimeClient {
    pub fn new(client: reqwest::Client, host: String, token: String) -> Self {
        CrimeClient {
            host,
            client,
            token: XAppToken(token),
        }
    }

    pub fn request(&self, query: Query) -> Result<String, String> {
        let url = format!("{}/{}?{}", self.host, "resource/policereport.json", query);
        self.client
            .get(&url)
            .header(self.token.clone())
            .send()
            .and_then(|mut r| r.text())
            .map_err(|e| format!("{}", e))
    }
}
