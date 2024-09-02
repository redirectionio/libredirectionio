use cidr::AnyIpCidr;
use serde::{Deserialize, Serialize};
use std::fmt::Display;
use std::net::IpAddr;

#[derive(Clone, Debug, Hash, Serialize, Deserialize, Eq, PartialEq)]
pub enum RouteIp {
    InRange(AnyIpCidr),
    NotInRange(AnyIpCidr),
}

impl RouteIp {
    pub fn match_ip(&self, ip: &IpAddr) -> bool {
        match self {
            Self::InRange(in_range) => in_range.contains(ip),
            Self::NotInRange(not_in_range) => !not_in_range.contains(ip),
        }
    }
}

impl Display for RouteIp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            Self::InRange(in_range) => format!("in({in_range})"),
            Self::NotInRange(not_in_range) => format!("not_in({not_in_range})"),
        };
        write!(f, "{}", str)
    }
}
