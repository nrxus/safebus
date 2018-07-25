# safebus
API to get bus information in seattle

## Routes

### Get Bus Stops Nearby

https://safebus.pcfbeta.io/api/bus_stops?lat={lat}lon={lon}&lat_span={lat_span}&lon_span={lon_span}

Example: https://safebus.pcfbeta.io/api/bus_stops?lat=47.653435&lon=-122.305641&lat_span=0.01&lon_span=0.01

### Get Departures + Crimes for Bus Stop

https://safebus.pcfbeta.io/api/bus_stop_status/{bus_stop_id}

Example: https://safebus.pcfbeta.io/api/bus_stop_status/98_755003

Run unit tests:
`cargo test`

Run contract tests:
`cargo contract-test`
