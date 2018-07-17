use client::Client;

use rocket::{response::content::Json, State};

#[derive(FromForm, Debug, Clone, Copy, PartialEq)]
pub struct Location {
    pub latitude: f64,
    pub longitude: f64,
}

#[get("/info?<location>")]
fn info(location: Location, client: State<Client>) -> Result<Json<String>, String> {
    client.info(location).map(Json)
}

// If run using `cargo test` then they will be run in "unit test" mode and use a mock client
// If run using `cargo contract-test` then they will be run in "contract test" mode and do a real network call
#[cfg(test)]
mod test {
    use super::*;

    use mocktopus::mocking::{MockResult, Mockable};
    use rocket::http::{ContentType, Status};

    #[test]
    fn info_route() {
        let mut location: Option<Location> = None;
        unsafe {
            Client::info.mock_raw(|_, loc| {
                location = Some(loc);
                MockResult::Return(Ok("Hello".to_string()))
            });
        }

        let client = ::client();
        let response = client
            .get("/api/info?latitude=-122.33&longitude=47.59")
            .dispatch();
        let location = location.expect("Client::Info not called");
        assert_eq!(
            location,
            Location {
                latitude: -122.33,
                longitude: 47.59
            }
        );
        assert_eq!(response.status(), Status::Ok);
        assert_eq!(response.content_type(), Some(ContentType::JSON));
    }
}
