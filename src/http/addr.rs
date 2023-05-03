use std::net::{IpAddr, SocketAddr};
use std::str::FromStr;

pub struct Addr {
    pub addr: IpAddr,
    pub port: Option<u16>,
}

impl std::fmt::Display for Addr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let ipv = if self.addr.is_ipv4() { "4" } else { "6" };

        let hex = match self.addr {
            IpAddr::V4(addr) => u32::from(addr) as u128,
            IpAddr::V6(addr) => u128::from(addr),
        };

        match self.port {
            Some(p) => write!(f, "address: {}, port: {}, hex: {:x} (IPv{})", self.addr, p, hex, ipv),

            None => write!(f, "address: {}, port: N/A, hex: {:x} (IPv{}) ", self.addr, hex, ipv),
        }
    }
}

impl FromStr for Addr {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let trimmed = s.trim_matches(|c| c == '\0' || c == '\n' || c == '\r' || c == '\t' || c == ' ');

        if let Ok(addr) = trimmed.parse::<IpAddr>() {
            Ok(Self { addr, port: None })
        } else if let Ok(sock) = trimmed.parse::<SocketAddr>() {
            Ok(Self {
                addr: sock.ip(),
                port: Some(sock.port()),
            })
        } else {
            Err(())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_invalid() {
        let addr = "invalid".parse::<Addr>();

        assert!(addr.is_err());
    }

    #[test]
    fn test_ipv4_without_port() {
        let addr = "127.0.0.1".parse::<Addr>().unwrap();

        assert!(addr.addr.is_ipv4());
        assert!(addr.port.is_none());
        assert_eq!(addr.addr.to_string(), "127.0.0.1");
    }

    #[test]
    fn test_ipv4_with_null() {
        let addr = "127.0.0.1\0".parse::<Addr>().unwrap();

        assert!(addr.addr.is_ipv4());
        assert!(addr.port.is_none());
        assert_eq!(addr.addr.to_string(), "127.0.0.1");
    }

    #[test]
    fn test_ipv4_with_port() {
        let addr = "127.0.0.1:8080".parse::<Addr>().unwrap();

        assert!(addr.addr.is_ipv4());
        assert!(addr.port.is_some());
        assert_eq!(addr.port.unwrap(), 8080);
        assert_eq!(addr.addr.to_string(), "127.0.0.1");
    }

    #[test]
    fn test_ipv6_without_port() {
        let addr = "::1".parse::<Addr>().unwrap();

        assert!(addr.addr.is_ipv6());
        assert!(addr.port.is_none());
        assert_eq!(addr.addr.to_string(), "::1");
    }

    #[test]
    fn test_ipv6_with_port() {
        let addr = "[::1]:8080".parse::<Addr>().unwrap();

        assert!(addr.addr.is_ipv6());
        assert!(addr.port.is_some());
        assert_eq!(addr.port.unwrap(), 8080);
        assert_eq!(addr.addr.to_string(), "::1");
    }
}
