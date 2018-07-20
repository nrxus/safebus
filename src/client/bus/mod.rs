use reqwest;

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct StopsQuery {
    pub lat: f64,
    pub lon: f64,
    pub lat_span: f64,
    pub lon_span: f64,
    pub max_count: i8,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StopInfo {
    pub id: String,
    pub direction: String,
    pub lat: f64,
    pub lon: f64,
    pub name: String,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct Status {
    pub route: String,
    pub headsign: String,
    pub scheduled_time: u64,
    pub predicted_time: u64,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Departures {
    pub buses: Vec<Status>,
    pub stop: StopInfo,
}

pub struct Client {
    host: String,
    key_query: [(&'static str, String); 1],
    http_client: reqwest::Client,
}

impl Client {
    pub fn new(http_client: reqwest::Client, host: String, key: String) -> Self {
        Client {
            host,
            http_client,
            key_query: [("key", key)],
        }
    }
}

// allow users of Client to mock the requests in unit tests
#[cfg(all(test, not(feature = "contract")))]
use mocktopus::macros::mockable;

#[cfg_attr(all(test, not(feature = "contract")), mockable)]
impl Client {
    pub fn stops(&self, query: &StopsQuery) -> Result<Vec<StopInfo>, String> {
        let url = format!("{}/api/where/stops-for-location.json", self.host);
        self.http_client
            .get(&url)
            .query(query)
            .query(&self.key_query)
            .send()
            .and_then(|mut r| r.json())
            .map(|r: StopsListResponse| r.data.list)
            .map_err(|e| format!("{}", e))
    }

    pub fn departures(&self, stop_id: &str) -> Result<Departures, String> {
        let url = format!(
            "{}/api/where/arrivals-and-departures-for-stop/{}.json",
            self.host, stop_id
        );
        self.http_client
            .get(&url)
            .query(&self.key_query)
            .send()
            .and_then(|mut r| r.json())
            .map(DeparturesResponse::into)
            .map_err(|e| format!("{}", e))
    }
}

impl From<DeparturesResponse> for Departures {
    fn from(r: DeparturesResponse) -> Departures {
        let data = r.data;
        let stop_id = data.entry.stop_id;
        let stops = data.references.stops;
        let departures = data.entry.arrivals_and_departures;

        let stop = stops.into_iter().find(|s| s.id == stop_id).unwrap();
        let buses = departures
            .into_iter()
            .map(|b| Status {
                headsign: b.trip_headsign,
                route: b.route_short_name,
                predicted_time: b.predicted_departure_time,
                scheduled_time: b.scheduled_departure_time,
            })
            .collect();
        Departures { stop, buses }
    }
}

#[derive(Deserialize)]
struct StopsListResponse {
    data: StopsListData,
}

#[derive(Deserialize)]
struct StopsListData {
    list: Vec<StopInfo>,
}

#[derive(Deserialize)]
struct DeparturesResponse {
    data: DeparturesListData,
}

#[derive(Deserialize)]
struct DeparturesListData {
    entry: DeparturesListEntry,
    references: DeparturesReferences,
}

#[derive(Deserialize)]
struct DeparturesReferences {
    stops: Vec<StopInfo>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct DeparturesListEntry {
    arrivals_and_departures: Vec<ArrivalAndDeparture>,
    stop_id: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct ArrivalAndDeparture {
    predicted_departure_time: u64,
    scheduled_departure_time: u64,
    route_short_name: String,
    trip_headsign: String,
}

#[cfg(all(test, not(feature = "contract")))]
mod test;
