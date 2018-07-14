#[cfg(any(not(test), feature = "contract"))]
mod real {
    use client;

    use std::env;

    pub fn crime_client() -> client::CrimeClient {
        let token =
            env::var("SEATTLE_API_KEY").expect("'SEATTLE_API_KEY' ENV VARIABLE IS REQUIRED");
        let host = "https://data.seattle.gov/";
        client::CrimeClient::new(host.into(), token)
    }
}

#[cfg(any(not(test), feature = "contract"))]
pub use self::real::*;

#[cfg(all(test, not(feature = "contract")))]
mod mocks {
    use client;
    use mockito;

    pub fn crime_client() -> client::CrimeClient {
        client::CrimeClient::new(mockito::SERVER_URL.into(), "SEATTLE_API_KEY".into())
    }
}

#[cfg(all(test, not(feature = "contract")))]
pub use self::mocks::*;
