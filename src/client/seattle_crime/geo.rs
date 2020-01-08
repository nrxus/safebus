mod earth_area;

use self::earth_area::EarthArea;

use geo_types::Polygon;
use geojson::GeoJson;
use std::convert::{TryFrom, TryInto};

#[derive(Debug, Clone, PartialEq, Copy)]
pub struct Location {
    pub lon: f64,
    pub lat: f64,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Beat {
    pub name: String,
    pub area_km: f64,
}

#[cfg_attr(test, faux::create)]
pub struct Client {
    http_client: reqwest::Client,
    host: String,
}

#[cfg_attr(test, faux::methods)]
impl Client {
    pub fn new(http_client: reqwest::Client, host: String) -> Self {
        Client { http_client, host }
    }
    
    pub fn beat_for(&self, location: Location) -> Result<Beat, String> {
        let url = format!(
            "{}/ArcGIS/rest/services/DoIT_ext/SP_Precincts_Beats/MapServer/2/query",
            self.host
        );
	
        let geo_json: GeoJson = self
            .http_client
            .get(url.as_str())
            .query(&[
                (
                    "geometry",
                    format!("{lon},{lat}", lon = location.lon, lat = location.lat).as_str(),
                ),
                ("geometryType", "esriGeometryPoint"),
                ("inSR", "4326"),
                ("returnGeometry", "true"),
                ("f", "geojson"),
            ])
            .send()
            .and_then(reqwest::Response::error_for_status)
            .and_then(|mut r| r.json())
            .map_err(|e| format!("{}", e))?;

        geo_json.try_into()
    }
}

impl TryFrom<GeoJson> for Beat {
    type Error = String;

    fn try_from(geo_json: GeoJson) -> Result<Beat, String> {
        let mut features = match geo_json {
            GeoJson::FeatureCollection(f) => Ok(f.features),
            _ => Err(format!("Expected FeatureCollection. Got {}", geo_json)),
        }?;
        let feature = features.pop().ok_or("empty features in collection")?;
        let (geometry, mut properties) = match (feature.geometry, feature.properties) {
            (Some(g), Some(p)) => Ok((g.value, p)),
            (None, Some(_)) => Err("Empty geometry in feature"),
            (Some(_), None) => Err("Empty properties in feature"),
            (None, None) => Err("Empty geometry + properties in feature"),
        }?;
        let polygon: Polygon<f64> = geometry
            .try_into()
            .map_err(|_| "could not convert geometry to polygon")?;
        let beat = properties
            .remove("beat")
            .ok_or("could not find 'beat' key in properties map")?;
        let beat = beat
            .as_str()
            .ok_or("could not parse 'beat' value as a string")?;
        Ok(Beat {
            name: beat.to_string(),
            area_km: polygon.area(),
        })
    }
}

#[cfg(all(test, not(feature = "integration")))]
mod test;
