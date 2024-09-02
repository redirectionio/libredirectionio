use chrono::{DateTime, NaiveTime, Utc};
use serde::{Deserialize, Serialize};
use std::fmt::Display;

#[derive(Clone, Debug, Hash, Serialize, Deserialize, Eq, PartialEq, Ord, PartialOrd)]
pub struct RouteTime {
    pub start: Option<NaiveTime>,
    pub end: Option<NaiveTime>,
}

impl RouteTime {
    pub fn from_range(start: &Option<String>, end: &Option<String>) -> RouteTime {
        let mut route_start = None;
        let mut route_end = None;
        match start {
            None => (),
            Some(time) => match time.parse::<NaiveTime>() {
                Ok(dt) => route_start = Some(dt),
                Err(err) => {
                    log::error!("cannot parse time {}: {}", time, err);
                }
            },
        }
        match end {
            None => (),
            Some(time) => match time.parse::<NaiveTime>() {
                Ok(dt) => route_end = Some(dt),
                Err(err) => {
                    log::error!("cannot parse time {}: {}", time, err);
                }
            },
        }

        RouteTime {
            start: route_start,
            end: route_end,
        }
    }

    pub fn match_datetime(&self, datetime: &DateTime<Utc>) -> bool {
        let naive_time = datetime.naive_utc().time();
        match self.start {
            None => match self.end {
                None => true,
                Some(end) => naive_time < end,
            },
            Some(start) => match self.end {
                None => naive_time >= start,
                Some(end) => (naive_time >= start) && (naive_time < end),
            },
        }
    }
}

impl Display for RouteTime {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self.start {
            None => match self.end {
                None => "always".to_string(),
                Some(end) => format!("before({})", end.format("%H:%M:%S")),
            },
            Some(start) => match self.end {
                None => format!("after({})", start.format("%H:%M:%S")),
                Some(end) => format!("in({}, {})", start.format("%H:%M:%S"), end.format("%H:%M:%S")),
            },
        };
        write!(f, "{}", str)
    }
}
