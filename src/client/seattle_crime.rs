pub mod data;
pub mod geo;

pub use self::{
    data::{Filter::*, Query},
    geo::Location,
};

use chrono::{Duration, Local};

#[derive(Debug, PartialEq)]
pub struct CrimeData {
    pub related_crimes: Vec<Crime>,
    pub unrelated_crimes: Vec<Crime>,
}

#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Crime {
    pub description: String,
    pub density: f64,
}

#[cfg_attr(test, faux::create)]
pub struct Service {
    data_client: data::Client,
    geo_client: geo::Client,
}

#[cfg_attr(test, faux::methods)]
impl Service {
    pub fn new(data_client: data::Client, geo_client: geo::Client) -> Self {
        Service {
            data_client,
            geo_client,
        }
    }

    pub fn crime_nearby(&self, location: Location) -> Result<CrimeData, String> {
        let beat = self.geo_client.beat_for(location)?;
        let three_months_ago = Local::now() - Duration::days(90);
        let area = beat.area_km;
        let crimes = self
            .data_client
            .crimes(&Query::new(After(three_months_ago)).and(Beat(beat.name)))?;
        let crimes = crimes.into_iter().map(|c| Crime {
            description: c.description,
            density: f64::from(c.count) / area,
        });

        let (related_crimes, unrelated_crimes) =
            crimes.fold((vec![], vec![]), |(mut related, mut unrelated), c| {
                if UNRELATED_CRIMES.iter().any(|&u| u == c.description) {
                    unrelated.push(c);
                } else if RELATED_CRIMES.iter().any(|&r| r == c.description) {
                    related.push(c);
                } else {
                    // not in one of the crimes we know about - add to unrelated and log
                    println!("GOT AN UNMATCHED CRIME: {}", c.description);
                    unrelated.push(c);
                }
                (related, unrelated)
            });

        Ok(CrimeData {
            related_crimes,
            unrelated_crimes,
        })
    }
}

const UNRELATED_CRIMES: [&str; 8] = [
    "FRAUD OFFENSES",
    "ARSON",
    "DRIVING UNDER THE INFLUENCE",
    "DRUG/NARCOTIC OFFENSES",
    "MOTOR VEHICLE THEFT",
    "DESTRUCTION/DAMAGE/VANDALISM OF PROPERTY",
    "TRESPASS OF REAL PROPERTY",
    "BURGLARY/BREAKING&ENTERING",
];

const RELATED_CRIMES: [&str; 6] = [
    "STOLEN PROPERTY OFFENSES",
    "LARCENY-THEFT",
    "SEX OFFENSES",
    "ROBBERY",
    "WEAPON LAW VIOLATIONS",
    "ASSAULT OFFENSES",
];

#[cfg(all(test, not(feature = "integration")))]
mod test;
