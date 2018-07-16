use client::Client;

use rocket::{response::content::Json, State};

#[derive(FromForm, Debug, Clone, Copy)]
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
    use rocket::http::{ContentType, Status};

    #[test]
    fn info_route() {
        let client = ::client();
        let response = client
            .get("/api/info?latitude=-122.33&longitude=47.59")
            .dispatch();
        assert_eq!(response.status(), Status::Ok);
        assert_eq!(response.content_type(), Some(ContentType::JSON));
    }
}
