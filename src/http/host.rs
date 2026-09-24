/// 80 and 443 are dropped from a host whatever the scheme, any other port is kept (`host:8080` rules).
pub fn is_default_port(port: &str) -> bool {
    port == "80" || port == "443"
}

/// Removes a default port from `host[:port]`, the result is a prefix of the input.
pub fn strip_default_port(host: &str) -> &str {
    let (name, port) = if host.starts_with('[') {
        // bracketed IPv6, "[::1]:443"
        match host.find(']') {
            Some(end) => match host[end + 1..].strip_prefix(':') {
                Some(port) => (&host[..=end], port),
                None => return host,
            },
            None => return host,
        }
    } else {
        // several colons is an unbracketed IPv6 address, left untouched
        match host.rsplit_once(':') {
            Some((name, port)) if !name.is_empty() && !name.contains(':') => (name, port),
            _ => return host,
        }
    };

    if is_default_port(port) { name } else { host }
}

/// Owned variant of [`strip_default_port`].
pub fn without_default_port(mut host: String) -> String {
    let len = strip_default_port(host.as_str()).len();
    host.truncate(len);

    host
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_ports_are_dropped_and_anything_else_is_left_untouched() {
        let cases = [
            ("example.org:443", "example.org"),
            ("example.org:80", "example.org"),
            ("EXAMPLE.ORG:443", "EXAMPLE.ORG"),
            ("example.org.:443", "example.org."),
            ("example.org:8080", "example.org:8080"),
            ("example.org", "example.org"),
            ("[::1]:443", "[::1]"),
            ("[::1]:80", "[::1]"),
            ("[::1]:8080", "[::1]:8080"),
            ("[::1]", "[::1]"),
            ("::1", "::1"),
            ("fe80::443", "fe80::443"),
            ("@domain:443", "@domain"),
            ("port-@port.example.org:443", "port-@port.example.org"),
            ("", ""),
            (":443", ":443"),
            ("example.org:", "example.org:"),
            ("example.org:0443", "example.org:0443"),
            ("example.org:443x", "example.org:443x"),
        ];

        for (input, expected) in cases {
            assert_eq!(strip_default_port(input), expected, "strip_default_port({input:?})");
            assert_eq!(without_default_port(input.to_string()), expected, "without_default_port({input:?})");
        }
    }
}
