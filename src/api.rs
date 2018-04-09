use service;

use reqwest::Client;
use rocket::{State, response::content::Json};

#[derive(FromForm, Debug, Clone, Copy)]
pub struct Location {
    pub latitude: f64,
    pub longitude: f64,
}

#[get("/info?<location>")]
fn info(location: Location, client: State<Client>) -> Result<Json<String>, String> {
    service::info(location, &*client).map(Json)
}

#[cfg(test)]
mod tests {
    use rocket::http::{ContentType, Status};

    // this test will do a real network call
    // exclude from most test runs to avoid a long test feedback cycle
    #[test]
    fn integration() {
        let client = ::client();
        let response = client
            .get("/api/info?latitude=-122.33&longitude=47.59")
            .dispatch();
        assert_eq!(response.status(), Status::Ok);
        assert_eq!(response.content_type(), Some(ContentType::JSON));
    }
}
