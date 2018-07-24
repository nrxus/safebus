use chrono::{DateTime, Local};

use std::fmt;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Filter {
    After(DateTime<Local>),
    Beat(String),
}

#[derive(Debug, PartialEq, Eq, Clone, Serialize)]
pub struct Query {
    #[serde(rename = "$where")]
    filters: String,
}

impl Query {
    pub fn new(filter: Filter) -> Self {
        Query {
            filters: filter.to_string(),
        }
    }

    pub fn and(self, filter: Filter) -> Self {
        Query {
            filters: format!("{} AND {}", self.filters, filter.to_string()),
        }
    }
}

impl fmt::Display for Filter {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Filter::After(date) => write!(f, "occ_datetime>'{}'", date.format("%Y-%m-%d")),
            Filter::Beat(beat) => write!(f, "beat='{}'", beat),
        }
    }
}

#[cfg(all(test, not(feature = "contract")))]
mod test {
    use super::*;

    use std::fmt::Display;

    use chrono::{NaiveDate, TimeZone};
    use serde_urlencoded;
    use url::form_urlencoded::byte_serialize;

    #[test]
    fn date_filter() {
        let date = NaiveDate::from_ymd(2014, 7, 24).and_hms(12, 34, 6);
        let filter = Filter::After(Local.from_local_datetime(&date).unwrap());
        let expected = "occ_datetime>'2014-07-24'";
        assert_eq!(filter.to_string(), expected);
    }

    #[test]
    fn beat_filter() {
        let filter = Filter::Beat(String::from("U3"));
        let expected = "beat='U3'";
        assert_eq!(filter.to_string(), expected);
    }

    #[test]
    fn query_serializes() {
        use super::Filter::*;

        let after_date = After(Local::now());
        let beat = Beat(String::from("U1"));
        let query = Query::new(after_date.clone()).and(beat.clone());
        let expected = format!(
            "{}={}",
            encode("$where"),
            encode(format!(
                "{} AND {}",
                after_date.to_string(),
                beat.to_string()
            ))
        );
        let actual = serde_urlencoded::to_string(query).unwrap();
        assert_eq!(actual, expected);
    }

    fn encode(input: impl Display) -> String {
        byte_serialize(input.to_string().as_bytes()).collect()
    }
}
