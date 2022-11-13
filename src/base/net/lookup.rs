use crate::rtps::common::error::*;
use trust_dns_resolver::config::*;
use trust_dns_resolver::Resolver;

pub type Err = RtpsError;

pub fn lookup_ipv4(name: &str) -> Result<String, RtpsError> {
    let resolver = Resolver::new(ResolverConfig::default(), ResolverOpts::default()).unwrap();

    let response = resolver.ipv4_lookup(name);
    match response {
        Ok(r) => match r.iter().next() {
            Some(address) => Ok(address.to_string()),
            None => Err(RtpsError::new("Ipv4 lookup")),
        },
        Err(_e) => Err(RtpsError::new("Ipv4 lookup")),
    }
}

pub fn lookup_ipv6(name: &str) -> Result<String, RtpsError> {
    let resolver = Resolver::new(ResolverConfig::default(), ResolverOpts::default()).unwrap();

    let response = resolver.ipv6_lookup(name);
    match response {
        Ok(r) => match r.iter().next() {
            Some(address) => Ok(address.to_string()),
            None => Err(RtpsError::new("Ipv4 lookup")),
        },
        Err(_e) => Err(RtpsError::new("Ipv4 lookup")),
    }
}
