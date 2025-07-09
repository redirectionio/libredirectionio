use std::{fmt::Display, net::IpAddr};

use cidr::AnyIpCidr;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Hash, Serialize, Deserialize, Eq, PartialEq)]
pub enum RouteIp {
    InRange(AnyIpCidr),
    NotInRange(AnyIpCidr),
    NotOneOf(Vec<IpAddr>),
}

impl RouteIp {
    pub fn match_ip(&self, ip: &IpAddr) -> bool {
        match self {
            Self::InRange(in_range) => in_range.contains(ip),
            Self::NotInRange(not_in_range) => !not_in_range.contains(ip),
            Self::NotOneOf(disallowed_ips) => !disallowed_ips.contains(ip),
        }
    }
}

impl Display for RouteIp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            Self::InRange(in_range) => format!("in({in_range})"),
            Self::NotInRange(not_in_range) => format!("not_in({not_in_range})"),
            Self::NotOneOf(list) => {
                let ips = list.iter().map(ToString::to_string).collect::<Vec<_>>().join(", ");
                format!("not_one_of([{ips}])")
            }
        };
        write!(f, "{str}")
    }
}
