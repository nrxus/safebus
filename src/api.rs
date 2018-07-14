use client::CrimeClient;
use service::Service;

use rocket::{Outcome, Request, State, request::{self, FromRequest}, response::content::Json};

impl<'a, 'r> FromRequest<'a, 'r> for Service<'r> {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<Self, Self::Error> {
        let client = request.guard::<State<CrimeClient>>()?.inner();
        Outcome::Success(Service { client })
    }
}

#[derive(FromForm, Debug, Clone, Copy)]
pub struct Location {
    pub latitude: f64,
    pub longitude: f64,
}

#[get("/info?<location>")]
fn info<'a>(location: Location, service: Service<'a>) -> Result<Json<String>, String> {
    service.info(location).map(Json)
}

// If run using `cargo test` then they will be run in "unit test" mode and use a mock client
// If run using `cargo contract-test` then they will be run in "contract test" mode and do a real network call
#[cfg(test)]
mod tests {
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
