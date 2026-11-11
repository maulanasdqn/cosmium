use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};

pub fn targets_private_network(url: &str) -> bool {
    host_of(url).is_some_and(is_private_host)
}

pub fn is_private_host(host: &str) -> bool {
    let host = host.trim_end_matches('.').to_ascii_lowercase();
    if host == "localhost" || host.ends_with(".localhost") {
        return true;
    }
    match host.parse::<IpAddr>() {
        Ok(IpAddr::V4(ip)) => is_private_v4(ip),
        Ok(IpAddr::V6(ip)) => is_private_v6(ip),
        Err(_) => false,
    }
}

fn host_of(url: &str) -> Option<&str> {
    let (_, rest) = url.split_once("://")?;
    let authority = rest.split(['/', '?', '#']).next()?;
    let host_port = authority.rsplit_once('@').map_or(authority, |(_, h)| h);
    if let Some(bracketed) = host_port.strip_prefix('[') {
        return bracketed.split_once(']').map(|(h, _)| h);
    }
    Some(host_port.split_once(':').map_or(host_port, |(h, _)| h))
}

fn is_private_v4(ip: Ipv4Addr) -> bool {
    let [first, second, ..] = ip.octets();
    let carrier_nat = first == 100 && (64..=127).contains(&second);
    ip.is_private()
        || ip.is_loopback()
        || ip.is_link_local()
        || ip.is_unspecified()
        || ip.is_broadcast()
        || carrier_nat
}

fn is_private_v6(ip: Ipv6Addr) -> bool {
    if let Some(mapped) = ip.to_ipv4_mapped() {
        return is_private_v4(mapped);
    }
    ip.is_loopback() || ip.is_unspecified() || ip.is_unique_local() || ip.is_unicast_link_local()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn blocks_every_private_ipv4_range() {
        for url in [
            "http://10.0.0.1/",
            "http://172.16.0.1/",
            "http://172.20.0.1/",
            "http://172.31.255.255/",
            "http://192.168.1.1/",
            "http://169.254.169.254/latest/meta-data/",
            "http://100.64.0.1/",
            "http://100.127.255.255/",
            "http://127.0.0.1:8080/admin",
            "http://0.0.0.0/",
        ] {
            assert!(targets_private_network(url), "{url}");
        }
    }

    #[test]
    fn allows_public_neighbours_of_private_ranges() {
        for url in [
            "https://example.com/",
            "http://172.15.0.1/",
            "http://172.32.0.1/",
            "http://100.63.0.1/",
            "http://100.128.0.1/",
            "http://8.8.8.8/",
            "https://[2606:4700::1111]/",
        ] {
            assert!(!targets_private_network(url), "{url}");
        }
    }

    #[test]
    fn blocks_localhost_names() {
        assert!(targets_private_network("http://localhost:3000/"));
        assert!(targets_private_network("http://LOCALHOST./"));
        assert!(targets_private_network("http://api.localhost/"));
        assert!(!targets_private_network("http://localhost.example.com/"));
    }

    #[test]
    fn blocks_private_ipv6() {
        for url in [
            "http://[::1]/",
            "http://[::1]:9222/json",
            "http://[fd00::1]/",
            "http://[fe80::1]/",
            "http://[::ffff:10.0.0.1]/",
            "http://[::ffff:169.254.169.254]/",
        ] {
            assert!(targets_private_network(url), "{url}");
        }
    }

    #[test]
    fn userinfo_cannot_hide_the_host() {
        assert!(targets_private_network("http://example.com@127.0.0.1/"));
        assert!(!targets_private_network("http://127.0.0.1@example.com/"));
    }

    #[test]
    fn ignores_non_urls() {
        assert!(!targets_private_network("about:blank"));
        assert!(!targets_private_network("data:text/html,hi"));
    }
}
