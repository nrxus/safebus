use query::Query;
use reqwest;

const API_TOKEN: &'static str = "5SrzVINmberri7ZBf0QytGtY5";

header! { (XAppToken, "X-App-Token") => [String]}

pub trait CrimeClient {
    fn request(&self, query: Query) -> Result<String, String>;
}

// No unit tests due to lack of mocking ability for reqwest::Client - CHANGE WITH CARE
impl CrimeClient for reqwest::Client {
    fn request(&self, query: Query) -> Result<String, String> {
        const BASE_URL: &'static str = "https://data.seattle.gov/resource/policereport.json";
        let url = format!("{}?{}", BASE_URL, query);
        self.get(&url)
            .header(XAppToken(API_TOKEN.into()))
            .send()
            .and_then(|mut r| r.text())
            .map_err(|e| format!("{}", e))
    }
}

// Mock for Client Trait
#[cfg(test)]
mod mocks {
    use super::*;

    use std::cell::{Ref, RefCell};

    #[derive(Debug)]
    pub struct Mock {
        requests: RefCell<Vec<Query>>,
        response: Result<String, String>,
    }

    impl Mock {
        pub fn new(response: Result<String, String>) -> Self {
            Mock {
                response,
                requests: Default::default(),
            }
        }

        pub fn requests(&self) -> Ref<[Query]> {
            Ref::map(self.requests.borrow(), |v| v.as_slice())
        }
    }

    impl CrimeClient for Mock {
        fn request(&self, query: Query) -> Result<String, String> {
            self.requests.borrow_mut().push(query);
            self.response.clone()
        }
    }
}

#[cfg(test)]
pub use self::mocks::Mock;
