#[cfg(any(not(test), feature = "contract"))]
mod real {
    use query::Query;
    use reqwest;

    const API_TOKEN: &'static str = "5SrzVINmberri7ZBf0QytGtY5";

    header! { (XAppToken, "X-App-Token") => [String]}

    pub struct CrimeClient(reqwest::Client);

    impl CrimeClient {
        pub fn new() -> Self {
            CrimeClient(reqwest::Client::new())
        }

        pub fn request(&self, query: Query) -> Result<String, String> {
            const BASE_URL: &'static str = "https://data.seattle.gov/resource/policereport.json";
            let url = format!("{}?{}", BASE_URL, query);
            self.0
                .get(&url)
                .header(XAppToken(API_TOKEN.into()))
                .send()
                .and_then(|mut r| r.text())
                .map_err(|e| format!("{}", e))
        }
    }
}

#[cfg(any(not(test), feature = "contract"))]
pub use self::real::*;

// Mock of Client for unit tests
#[cfg(all(test, not(feature = "contract")))]
mod mock {
    use query::Query;

    use std::cell::{Ref, RefCell};

    #[derive(Debug, Default)]
    pub struct CrimeClient {
        requests: RefCell<Vec<Query>>,
        response: Option<Result<String, String>>,
    }

    impl CrimeClient {
        pub fn new() -> Self {
            Self::default()
        }

        pub fn requests(&self) -> Ref<[Query]> {
            Ref::map(self.requests.borrow(), |v| v.as_slice())
        }

        pub fn set_response(&mut self, response: Result<String, String>) {
            self.response = Some(response)
        }

        pub fn request(&self, query: Query) -> Result<String, String> {
            self.requests.borrow_mut().push(query);
            self.response.clone().unwrap()
        }
    }

    unsafe impl Sync for CrimeClient {}
}

#[cfg(all(test, not(feature = "contract")))]
pub use self::mock::*;
