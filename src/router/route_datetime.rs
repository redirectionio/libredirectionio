use std::fmt::Display;

use chrono::{DateTime, NaiveDateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Hash, Serialize, Deserialize, Eq, PartialEq, Ord, PartialOrd)]
pub struct RouteDateTime {
    pub start: Option<NaiveDateTime>,
    pub end: Option<NaiveDateTime>,
}

impl RouteDateTime {
    pub fn from_range(start: &Option<String>, end: &Option<String>) -> RouteDateTime {
        let mut route_start = None;
        let mut route_end = None;
        match start {
            None => (),
            Some(datetime) => match datetime.parse::<DateTime<Utc>>() {
                Ok(dt) => route_start = Some(dt.naive_utc()),
                Err(err) => {
                    log::error!("cannot parse datetime {datetime}: {err}");
                }
            },
        }
        match end {
            None => (),
            Some(datetime) => match datetime.parse::<DateTime<Utc>>() {
                Ok(dt) => route_end = Some(dt.naive_utc()),
                Err(err) => {
                    log::error!("cannot parse datetime {datetime}: {err}");
                }
            },
        }

        RouteDateTime {
            start: route_start,
            end: route_end,
        }
    }

    pub fn match_datetime(&self, datetime: &DateTime<Utc>) -> bool {
        let naive_datetime = datetime.naive_utc();
        match self.start {
            None => match self.end {
                None => true,
                Some(end) => naive_datetime < end,
            },
            Some(start) => match self.end {
                None => naive_datetime >= start,
                Some(end) => (naive_datetime >= start) && (naive_datetime < end),
            },
        }
    }
}

impl Display for RouteDateTime {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self.start {
            None => match self.end {
                None => "always".to_string(),
                Some(end) => format!("before({})", end.format("%Y-%m-%d %H:%M:%S")),
            },
            Some(start) => match self.end {
                None => format!("after({})", start.format("%Y-%m-%d %H:%M:%S")),
                Some(end) => format!("in({}, {})", start.format("%Y-%m-%d %H:%M:%S"), end.format("%Y-%m-%d %H:%M:%S")),
            },
        };
        write!(f, "{str}")
    }
}
