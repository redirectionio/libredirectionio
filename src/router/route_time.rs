use serde::{Deserialize, Serialize};
use chrono::{NaiveTime, NaiveDateTime, DateTime, Utc};

#[derive(Clone, Debug, Hash, Serialize, Deserialize, Eq, PartialEq)]
pub struct RouteTime {
    pub start: Option<NaiveTime>,
    pub end: Option<NaiveTime>,
}

impl RouteTime {
    pub fn from_range(
        start: &Option<String>,
        end: &Option<String>,
    ) -> RouteTime {
        let mut route_start = None;
        let mut route_end = None;
        match start {
            None => (),
            Some(datetime) => {
                match datetime.parse::<NaiveDateTime>() {
                    Ok(dt) => route_start = Some(dt.time()),
                    Err(err) => {
                        log::error!("cannot parse datetime {}: {}", datetime, err);
                    }
                }
            }
        }
        match end {
            None => (),
            Some(datetime) => {
                match datetime.parse::<NaiveDateTime>() {
                    Ok(dt) => route_end = Some(dt.time()),
                    Err(err) => {
                        log::error!("cannot parse datetime {}: {}", datetime, err);
                    }
                }
            }
        }
        return RouteTime {
            start: route_start,
            end: route_end,
        };
    }

    pub fn match_datime(&self, datetime: &DateTime<Utc>) -> bool {
        let naive_time = datetime.naive_utc().time();
        match self.start {
            None => {
                match self.end {
                    None => true,
                    Some(end) => naive_time < end,
                }
            },
            Some(start) => {
                match self.end {
                    None => naive_time >= start,
                    Some(end) => (naive_time >= start) && (naive_time < end),
                }
            }
        }
    }
}

impl ToString for RouteTime {
    fn to_string(&self) -> String {
        match self.start {
            None => match self.end {
                None => "always".to_string(),
                Some(end) => format!("before({})", end.format("%H:%M:%S")),
            },
            Some(start) => match self.end {
                None => format!("after({})", start.format("%H:%M:%S")),
                Some(end) => format!("in({}, {})", start.format("%H:%M:%S"), end.format("%H:%M:%S")),
            },
        }
    }
}
