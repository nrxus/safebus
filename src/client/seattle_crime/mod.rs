pub mod data;
pub mod geo;

pub use self::data::{Filter::*, Query};
pub use self::geo::Location;

use chrono::{Duration, Local};

#[derive(Debug, PartialEq)]
pub struct CrimeData {
    pub related_crimes: Vec<Crime>,
    pub unrelated_crimes: Vec<Crime>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Crime {
    pub description: String,
    pub density: f64,
}

pub struct Service {
    data_client: data::Client,
    geo_client: geo::Client,
}

impl Service {
    pub fn new(data_client: data::Client, geo_client: geo::Client) -> Self {
        Service {
            data_client,
            geo_client,
        }
    }
}

// allow users of Service to mock the requests in unit tests
#[cfg(all(test, not(feature = "integration")))]
use mocktopus::macros::mockable;

#[cfg_attr(all(test, not(feature = "integration")), mockable)]
impl Service {
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

const UNRELATED_CRIMES: [&'static str; 163] = [
    "ADULT-VULNERABLE-NEGLECT",
    "ANIMAL-CRUELTY",
    "ANIMAL-OTH",
    "ARSON-BUSINESS",
    "ARSON-OTHER",
    "BIAS INCIDENT",
    "BRIBERY",
    "BURGLARY-FORCE-NONRES",
    "BURGLARY-FORCE-RES",
    "BURGLARY-NOFORCE-NONRES",
    "BURGLARY-NOFORCE-RES",
    "BURGLARY-OTHER",
    "BURGLARY-SECURE PARKING-NONRES",
    "BURGLARY-SECURE PARKING-RES",
    "CHILD-ABUSED-NOFORCE",
    "CHILD-HARBOR MINOR",
    "CHILD-NEGLECT",
    "CHILD-OTHER",
    "COUNTERFEIT",
    "DISORDERLY CONDUCT",
    "DISPUTE-CIVIL PROPERTY (AUTO)",
    "DISPUTE-CIVIL PROPERTY (NON AU",
    "DISPUTE-OTH",
    "DISTURBANCE-NOISE",
    "DISTURBANCE-OTH",
    "DRIVE-BY",
    "DUI-DRUGS",
    "DUI-LIQUOR",
    "ELUDING-FELONY FLIGHT",
    "EMBEZZLE",
    "ESCAPE",
    "EXTORTION",
    "FALSE REPORT",
    "FIREWORK-POSSESS",
    "FIREWORK-USE",
    "FORGERY-CHECK",
    "FORGERY-CREDIT CARD",
    "FORGERY-OTH",
    "FRAUD-CHECK",
    "FRAUD-COMPUTER",
    "FRAUD-CREDIT CARD",
    "FRAUD-IDENTITY THEFT",
    "FRAUD-OTHER",
    "FRAUD-WELFARE",
    "FRAUD-WIRE-ELECTRONIC",
    "GAMBLE-BETTING",
    "GAMBLE-EQUIPMENT",
    "GAMBLE-OPERATE",
    "GAMBLE-SPORTS TAMPERING",
    "HARBOR - BOATING UNDER INFLUEN",
    "HARBOR - CRIMINAL CODE VIOLATI",
    "ILLEGAL DUMPING",
    "INJURY - ACCIDENTAL",
    "INJURY - OTHER",
    "LIQUOR LAW VIOLATION",
    "LOITERING",
    "NARC-DISTRIBUTE-HALLUCINOGEN",
    "NARC-DRUG TRAFFIC LOITERING",
    "NARC-EQUIPMENT/PARAPHENALIA",
    "NARC-FORGERY-PRESCRIPTION",
    "NARC-FOUND-AMPHETAMINE",
    "NARC-FOUND-BARBITUATE",
    "NARC-FOUND-COCAINE",
    "NARC-FOUND-HALLUCINOGEN",
    "NARC-FOUND-HEROIN",
    "NARC-FOUND-MARIJU",
    "NARC-FOUND-METH",
    "NARC-FOUND-OPIUM",
    "NARC-FOUND-OTHER",
    "NARC-FOUND-PILL/TABLET",
    "NARC-FOUND-SYNTHETIC",
    "NARC-FRAUD-PRESCRIPTION",
    "NARC-MANUFACTURE-AMPHETAMINE",
    "NARC-MANUFACTURE-BARBITUATE",
    "NARC-MANUFACTURE-HALLUCINOGEN",
    "NARC-MANUFACTURE-METH",
    "NARC-MANUFACTURE-OTHER",
    "NARC-POSSESS-AMPHETAMINE",
    "NARC-POSSESS-BARBITUATE",
    "NARC-POSSESS-COCAINE",
    "NARC-POSSESS-HALLUCINOGEN",
    "NARC-POSSESS-HEROIN",
    "NARC-POSSESS-MARIJU",
    "NARC-POSSESS-METH",
    "NARC-POSSESS-OPIUM",
    "NARC-POSSESS-OTHER",
    "NARC-POSSESS-PILL/TABLET",
    "NARC-POSSESS-PRESCRIPTION",
    "NARC-POSSESS-SYNTHETIC",
    "NARC-PRODUCE-MARIJU",
    "NARC-SELL-AMPHETAMINE",
    "NARC-SELL-BARBITUATE",
    "NARC-SELL-COCAINE",
    "NARC-SELL-HALLUCINOGEN",
    "NARC-SELL-HEROIN",
    "NARC-SELL-MARIJU",
    "NARC-SELL-METH",
    "NARC-SELL-OPIUM",
    "NARC-SELL-OTHER",
    "NARC-SELL-PILL/TABLET",
    "NARC-SELL-PRESCRIPTION",
    "NARC-SELL-SYNTHETIC",
    "NARC-SMUGGLE-COCAINE",
    "NARC-SMUGGLE-HEROIN",
    "NARC-SMUGGLE-MARIJU",
    "NARC-SMUGGLE-METH",
    "NARC-SMUGGLE-OPIUM",
    "NARC-SMUGGLE-OTHER",
    "NARC-SMUGGLE-SYNTHETIC",
    "OBSTRUCT",
    "PORNOGRAPHY-OBSCENE MATERIAL",
    "PROP RECOVERED-OTHER AGENCY",
    "PROPERTY DAMAGE - GRAFFITI",
    "PROPERTY DAMAGE-NON RESIDENTIA",
    "PROPERTY DAMAGE-RESIDENTIAL",
    "PROPERTY FOUND",
    "PROPERTY LOST - POLICE EQUIPME",
    "PROPERTY LOST",
    "PROPERTY RECOVERED - POLICE EQ",
    "PROPERTY STOLEN - POLICE EQUIP",
    "PROPERTY STOLEN-CONCEAL",
    "PROPERTY STOLEN-POSSESS",
    "PROPERTY STOLEN-RECEIVE",
    "PROPERTY STOLEN-SELL",
    "PROPERTY STOLEN-TRAFFICKING",
    "PROSTITUTION LOITERING",
    "PROSTITUTION PATRONIZING",
    "PROSTITUTION",
    "PROSTITUTION-ASSIST-PROMOTE",
    "RECKLESS BURNING",
    "SEXOFF-SODOMY",
    "SOAP-VIOL - ZONE 1",
    "SOAP-VIOL - ZONE 2",
    "SOAP-VIOL - ZONE 3",
    "SOAP-VIOL - ZONE 4",
    "SOAP-VIOL - ZONE 5",
    "SOAP-VIOL - ZONE 6",
    "SOAP-VIOL - ZONE 7",
    "SODA-VIOL-EAST",
    "SODA-VIOL-NORTH",
    "SODA-VIOL-SOUTH",
    "SODA-VIOL-SOUTHWEST",
    "SODA-VIOL-WEST",
    "THEFT OF SERVICES",
    "THEFT-UNLAWFUL ISSUANCE OF BAN",
    "TRAFFIC",
    "TRESPASS",
    "UNLAWFUL USE OF MOTOR VEHICLE",
    "VEH-RCVD-FOR OTHER AGENCY",
    "VEH-THEFT-AIRCRAFT",
    "VEH-THEFT-AUTO",
    "VEH-THEFT-HVYEQUIP",
    "VEH-THEFT-MTRCYCLE",
    "VEH-THEFT-OTHVEH",
    "VEH-THEFT-RECREATION VEH",
    "VEH-THEFT-TRAILER",
    "VEH-THEFT-TRUCK",
    "VIOL-COURT ORDER",
    "WARRANT-FUGITIVE",
    "WARRARR-FELONY",
    "WARRARR-MISDEMEANOR",
    "WEAPON-SURRENDER-EXCLUDING FIR",
    "[INC - CASE DC USE ONLY]",
];

const RELATED_CRIMES: [&'static str; 73] = [
    "ANIMAL-BITE",
    "ASSLT-AGG-BODYFORCE",
    "ASSLT-AGG-DV-BODYFORCE",
    "ASSLT-AGG-DV-GUN",
    "ASSLT-AGG-DV-WEAPON",
    "ASSLT-AGG-GUN",
    "ASSLT-AGG-POLICE-BODYFORCE",
    "ASSLT-AGG-POLICE-GUN",
    "ASSLT-AGG-POLICE-WEAPON",
    "ASSLT-AGG-WEAPON",
    "ASSLT-NONAGG",
    "ASSLT-NONAGG-POLICE",
    "ASSLT-OTHER",
    "ASSLT-POLICE ANIMAL",
    "ENDANGER",
    "ENDANGERMENT",
    "HARASSMENT",
    "HOMICIDE-JUST-GUN",
    "HOMICIDE-JUST-STRONGARM",
    "HOMICIDE-JUST-WEAPON",
    "HOMICIDE-NEG-MANS-BODYFORCE",
    "HOMICIDE-NEG-MANS-GUN",
    "HOMICIDE-NEG-MANS-VEHICLE",
    "HOMICIDE-NEG-MANS-WEAPON",
    "HOMICIDE-PREMEDITATED-BODYFORC",
    "HOMICIDE-PREMEDITATED-GUN",
    "HOMICIDE-PREMEDITATED-WEAPON",
    "MALICIOUS HARASSMENT",
    "METRO TRANSIT - ON BUS, TUNNEL",
    "RAPE-STRONGARM",
    "ROBBERY-BANK-BODYFORCE",
    "ROBBERY-BANK-GUN",
    "ROBBERY-BANK-OTHER",
    "ROBBERY-BANK-WEAPON",
    "ROBBERY-BUSINESS-BODYFORCE",
    "ROBBERY-BUSINESS-GUN",
    "ROBBERY-BUSINESS-WEAPON",
    "ROBBERY-OTHER",
    "ROBBERY-RESIDENCE-BODYFORCE",
    "ROBBERY-RESIDENCE-GUN",
    "ROBBERY-RESIDENCE-WEAPON",
    "ROBBERY-STREET-BODYFORCE",
    "ROBBERY-STREET-GUN",
    "ROBBERY-STREET-WEAPON",
    "SEXOFF-INDECENT EXPOSURE",
    "SEXOFF-INDECENT LIBERTIES",
    "SEXOFF-LEWD CONDUCT",
    "SEXOFF-OTHER",
    "SEXOFF-PEEPER",
    "THEFT-AUTO PARTS",
    "THEFT-AUTOACC",
    "THEFT-BICYCLE",
    "THEFT-BOAT",
    "THEFT-BUILDING",
    "THEFT-CARPROWL",
    "THEFT-COINOP",
    "THEFT-LICENSE PLATE",
    "THEFT-MAIL",
    "THEFT-OTH",
    "THEFT-PKPOCKET",
    "THEFT-PRSNATCH",
    "THEFT-SHOPLIFT",
    "THREATS-DIGNITARY",
    "THREATS-KILL",
    "THREATS-OTHER",
    "THREATS-WEAPON",
    "URINATING/DEFECATING-IN PUBLIC",
    "VEH-THEFT-BUS",
    "WEAPON-CONCEALED",
    "WEAPON-DISCHARGE",
    "WEAPON-POSSESSION",
    "WEAPON-SELLING",
    "WEAPON-UNLAWFUL USE",
];

#[cfg(all(test, not(feature = "integration")))]
mod test;
