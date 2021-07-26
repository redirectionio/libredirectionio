use cidr::{AnyIpCidr, NetworkParseError};
use std::net::IpAddr;

#[derive(Debug, Clone)]
pub struct TrustedProxies(Vec<AnyIpCidr>);

impl Default for TrustedProxies {
    fn default() -> Self {
        Self(Vec::new())
    }
}

impl TrustedProxies {
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
