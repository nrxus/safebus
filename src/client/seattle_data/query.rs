use api::Location;

use chrono::{DateTime, Local};

use std::fmt;

#[derive(Debug, PartialEq, Eq, Clone, Serialize)]
pub struct Query {
    #[serde(rename = "$where")]
    filters: String,
}

impl Query {
    pub fn new(filter: impl Into<Filter>) -> Self {
        Query {
            filters: filter.into().to_string(),
        }
    }

    pub fn and(self, filter: impl Into<Filter>) -> Self {
        Query {
            filters: format!("{}%20AND%20{}", self.filters, filter.into()),
        }
    }
}

impl fmt::Display for Query {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "$where={}", self.filters)
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Filter(String);

impl fmt::Display for Filter {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(&self.0)
    }
}

impl From<Location> for Filter {
    fn from(location: Location) -> Filter {
        Filter(format!(
            "within_circle(location,{},{},150)",
            location.longitude, location.latitude
        ))
    }
}

impl From<DateTime<Local>> for Filter {
    fn from(date: DateTime<Local>) -> Self {
        let date = date.format("%Y-%m-%dT%H:%M:%S");
        Filter(format!("occurred_date_or_date_range_start>'{}'", date))
    }
}

#[cfg(all(test, not(feature = "contract")))]
mod test {
    use super::*;
    use chrono::{NaiveDate, TimeZone};

    #[test]
    fn location_filter() {
        let filter = Filter::from(Location {
            latitude: 35.6,
            longitude: -90.2,
        });
        let expected = "within_circle(location,-90.2,35.6,150)";
        assert_format(filter, expected);
    }

    #[test]
    fn date_filter() {
        let date = NaiveDate::from_ymd(2014, 7, 24).and_hms(12, 34, 6);
        let filter = Filter::from(Local.from_local_datetime(&date).unwrap());
        let expected = "occurred_date_or_date_range_start>'2014-07-24T12:34:06'";
        assert_format(filter, expected);
    }

    #[test]
    fn query() {
        let location = Location {
            latitude: 42.4,
            longitude: -28.3,
        };
        let date = Local::now();
        let query = Query::new(location).and(date);
        let expected = format!(
            "$where={}%20AND%20{}",
            Filter::from(location),
            Filter::from(date)
        );
        assert_format(query, &expected);
    }

    fn assert_format(actual: impl fmt::Display, expected: &str) {
        assert_eq!(format!("{}", actual), expected);
    }
}
