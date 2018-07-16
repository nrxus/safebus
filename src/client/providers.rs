use client::seattle_data;
use reqwest;

#[cfg(any(not(test), feature = "contract"))]
mod real {
    use super::*;

    use std::env;

    pub fn seattle_client(http_client: reqwest::Client) -> seattle_data::Client {
        let token =
            env::var("SEATTLE_API_KEY").expect("'SEATTLE_API_KEY' ENV VARIABLE IS REQUIRED");
        let host = "https://data.seattle.gov/".to_string();
        seattle_data::Client::new(http_client, host, token)
    }
}

#[cfg(any(not(test), feature = "contract"))]
pub use self::real::*;

#[cfg(all(test, not(feature = "contract")))]
mod mocks {
    use super::*;

    use mockito::{self, Matcher};

    use std::mem;

    pub fn seattle_client(http_client: reqwest::Client) -> seattle_data::Client {
        let token = "SEATTLE_API_KEY".to_string();
        let host = format!("{}/seattle_client", mockito::SERVER_URL);
        let mock = mockito::mock("GET", Matcher::Regex("r^/seattle_client.*$".to_string()))
            .with_status(200)
            .with_body("{}")
            .with_header("Content-Type", "application/json")
            .create();
        mem::forget(mock);

        seattle_data::Client::new(http_client, host, token)
    }
}

#[cfg(all(test, not(feature = "contract")))]
pub use self::mocks::*;
