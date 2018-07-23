pub mod data;

const NOT_RELEVANT_CRIMES: [&'static str; 46] = [
    "ANIMAL COMPLAINT",
    "BIAS INCIDENT",
    "COUNTERFEIT",
    "DISORDERLY CONDUCT",
    "DISPUTE",
    "DISTURBANCE",
    "DUI",
    "ELUDING",
    "EMBEZZLE",
    "ESCAPE",
    "EXTORTION",
    "FALSE REPORT",
    "FIREWORK",
    "FORGERY",
    "FRAUD AND FINANCIAL",
    "FRAUD",
    "GAMBLE",
    "HARBOR CALLS",
    "ILLEGAL DUMPING",
    "INJURY",
    "LIQUOR VIOLATION",
    "LOITERING",
    "LOST PROPERTY",
    "MAIL THEFT",
    "METRO",
    "NARCOTICS",
    "OBSTRUCT",
    "OTHER PROPERTY",
    "PORNOGRAPHY",
    "PROPERTY DAMAGE",
    "PROSTITUTION",
    "PUBLIC NUISANCE",
    "RECKLESS BURNING",
    "RECOVERED PROPERTY",
    "SHOPLIFTING",
    "STAY OUR OF AREA OF DRUGS",
    "STAY OUT OF AREA OF DRUGS",
    "STAY OUT OF AREA OF PROSTITUTION",
    "STOLEN PROPERTY",
    "THEFT OF SERVICES",
    "THREATS",
    "TRAFFIC",
    "TRESPASS",
    "VIOLATION OF COURT ORDER",
    "WARRANT ARREST",
    "[INC - CASE DC USE ONLY]",
];

const RELEVANT_CRIMES: [&'static str; 12] = [
    "ASSAULT",
    "ASSLT-POLICE ANIMAL",
    "BIKE THEFT",
    "BURGLARY",
    "BURGLARY-SECURE PARKING-RES",
    "CAR PROWL",
    "HOMICIDE",
    "PICKPOCKET",
    "PURSE SNATCH",
    "ROBBERY",
    "VEHICLE THEFT",
    "WEAPON",
];

pub struct Service {
    data_client: data::Client,
}

impl Service {
    pub fn new(data_client: data::Client) -> Self {
        Service { data_client }
    }
}
