use cidr::{errors::NetworkParseError, AnyIpCidr};
use std::net::IpAddr;
use std::str::FromStr;

#[derive(Debug, Clone, Default)]
pub struct TrustedProxies(Vec<AnyIpCidr>);

impl TrustedProxies {
    pub fn new_local() -> Self {
        Self(vec![
            // IPV4 Loopback
            AnyIpCidr::from_str("127.0.0.0/8").unwrap(),
            // IPV4 Private Networks
            AnyIpCidr::from_str("10.0.0.0/8").unwrap(),
            AnyIpCidr::from_str("172.16.0.0/12").unwrap(),
            AnyIpCidr::from_str("192.168.0.0/16").unwrap(),
            // IPV6 Loopback
            AnyIpCidr::from_str("::1/128").unwrap(),
            // IPV6 Private network
            AnyIpCidr::from_str("fd00::/8").unwrap(),
        ])
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn add_trusted_proxy(&mut self, proxy: &str) -> Result<(), NetworkParseError> {
        match proxy.parse() {
            Ok(v) => {
                self.0.push(v);

                Ok(())
            }
            Err(e) => Err(e),
        }
    }

    pub fn contains(&self, remote_addr: &IpAddr) -> bool {
        for proxy in &self.0 {
            if proxy.contains(remote_addr) {
                return true;
            }
        }

        false
    }

    pub fn remove_trusted_ips(&self, sorted_ips: Vec<IpAddr>) -> Vec<IpAddr> {
        let mut untrusted_sorted_ips = Vec::new();

        for ip in sorted_ips {
            if !self.contains(&ip) {
                untrusted_sorted_ips.push(ip);
            }
        }

        untrusted_sorted_ips
    }
}
