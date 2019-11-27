# Safebus API
REST API to get bus and crime information in Seattle

The API (in theory) runs in PCF at https://safebus.cfapps.io/.

The two available endpoints are:

- [getting bus stops](https://safebus.cfapps.io/api/bus_stops?lat=47.653435&lon=-122.305641&lat_span=0.01&lon_span=0.01) using `/api/bus_stops?lat={lat}lon={lon}&lat_span={lat_span}&lon_span={lon_span}`

- [getting single bus stop information](https://safebus.cfapps.io/api/bus_stops?lat=47.653435&lon=-122.305641&lat_span=0.01&lon_span=0.01) using `api/bus_stop_status/{bus_stop_id}`

Crime Data is obtained from the Seattle Crime Data API.
Bus Data is obtained from the One Bus Away API.

**NOTE:** the crime data API has not been updated since May, 2019. It will supposedly start getting updates again in Fall 2019 sometime. Until then, no crime data will show up.

## Development

Assuming you have installed Rust and Cargo using `rustup` you should be able to use the expected commands for local development

* `cargo run`. Opens a port on `localhost:8000`
* `cargo test`. Runs the unit tests

With the notable addition of `cargo integration-test` which will run a simple test that actually hits the underlying APIs and expects a succesful response.

Note that to run this you will need to set the following environment variables:

* `SEATTLE_API_KEY`
* `ONEBUSAWAY_API_KEY`
