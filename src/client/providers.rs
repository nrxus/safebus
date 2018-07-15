use client::crime_client::CrimeClient;
use reqwest;

#[cfg(any(not(test), feature = "contract"))]
mod real {
    use super::*;

    use std::env;

    pub fn crime_client(http_client: reqwest::Client) -> CrimeClient {
        let token =
            env::var("SEATTLE_API_KEY").expect("'SEATTLE_API_KEY' ENV VARIABLE IS REQUIRED");
        let host = "https://data.seattle.gov/".to_string();
        CrimeClient::new(http_client, host, token)
    }
}

#[cfg(any(not(test), feature = "contract"))]
pub use self::real::*;

#[cfg(all(test, not(feature = "contract")))]
mod mocks {
    use super::*;

    use mockito;

    pub fn crime_client(http_client: reqwest::Client) -> CrimeClient {
        let token = "SEATTLE_API_KEY".to_string();
        let host = mockito::SERVER_URL.to_string();
        CrimeClient::new(http_client, host, token)
    }
}

#[cfg(all(test, not(feature = "contract")))]
pub use self::mocks::*;
