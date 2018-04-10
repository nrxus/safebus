use client::CrimeClient;
use service;

use rocket::{State, response::content::Json};

#[derive(FromForm, Debug, Clone, Copy)]
pub struct Location {
    pub latitude: f64,
    pub longitude: f64,
}

#[get("/info?<location>")]
fn info(location: Location, client: State<CrimeClient>) -> Result<Json<String>, String> {
    service::info(location, &*client).map(Json)
}

// If run using `cargo test` then they will be run in "unit test" mode and use a mock client
// If run using `cargo contract-test` then they will be run in "contract test" mode and do a real network call
#[cfg(test)]
mod tests {
    use rocket::http::{ContentType, Status};

    #[cfg(not(feature = "contract"))]
    mod unit {
        use RocketExt;
        use client::CrimeClient;
        use rocket::Rocket;

        impl RocketExt for Rocket {
            fn inject(self) -> Self {
                let mut client = CrimeClient::new();
                client.set_response(Ok("[]".into()));
                self.manage(client)
            }
        }
    }

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
