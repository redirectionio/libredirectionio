use chrono::{DateTime, Datelike, Utc, Weekday};
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use std::fmt::{Display, Formatter, Result};

#[derive(Clone, Debug, Hash, Serialize, Deserialize, Eq, PartialEq)]
pub struct Weekdays(pub Vec<Weekday>);
#[derive(Clone, Debug, Hash, Serialize, Deserialize, Eq, PartialEq, Ord, PartialOrd)]
pub struct RouteWeekday {
    pub weekdays: Weekdays,
}

impl Display for Weekdays {
    fn fmt(&self, f: &mut Formatter) -> Result {
        self.0
            .iter()
            .fold(Ok(()), |result, weekday| result.and_then(|_| writeln!(f, "{}", weekday)))
    }
}

impl Ord for Weekdays {
    fn cmp(&self, other: &Self) -> Ordering {
        let self_num = self.0.iter().map(|weekday| weekday.num_days_from_monday());
        let other_num = other.0.iter().map(|weekday| weekday.num_days_from_monday());
        return self_num.cmp(other_num);
    }
}

impl PartialOrd for Weekdays {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl RouteWeekday {
    pub fn from_weekdays(weekdays: &Vec<String>) -> Option<RouteWeekday> {
        let mut route_weekdays = Vec::new();

        for weekday in weekdays {
            match weekday.parse::<Weekday>() {
                Ok(wd) => route_weekdays.push(wd),
                Err(err) => {
                    log::error!("cannot parse weekday {}: {}", weekday, err);
                }
            }
        }

        if route_weekdays.is_empty() {
            return None;
        } else {
            return Some(RouteWeekday {
                weekdays: Weekdays(route_weekdays),
            });
        }
    }

    pub fn match_datetime(&self, datetime: &DateTime<Utc>) -> bool {
        if (self.weekdays.0).contains(&datetime.weekday()) {
            return true;
        }
        return false;
    }
}

impl ToString for RouteWeekday {
    fn to_string(&self) -> String {
        format!("in({})", self.weekdays)
    }
}
