use crate::rtps::common::locator::*;
use crate::LOCATOR_ADDRESS_INVALID;
use regex::Regex;
use std::net::AddrParseError;
use std::net::Ipv4Addr;
use std::net::Ipv6Addr;
use std::str::FromStr;
use substring::Substring;

pub fn toIPv4string(locator: &Locator_t) -> String {
    format!(
        "{}.{}.{}.{}",
        locator.address[12], locator.address[13], locator.address[14], locator.address[15]
    )
}

pub fn toIPv6string(locator: &Locator_t) -> String {
    /* RFC 5952 Recommendation
     *  4.1     No leading zeros before each block
     *  4.2.1   Shorten as much as possible (do not leave 0 blocks at sides of ::)
     *  4.2.2   No collapse size 1 blocks
     *  4.2.3   Collapse the largest block (or the first one in case of tie)
     *  4.3     Lowercase
     */
    let mut max_block_index: usize = 0;
    let mut max_block_size: usize = 0;
    let mut actual_block_index: usize = 0;
    let mut actual_block_size: usize = 0;

    // 4.1, 4.3     By default using << std::hex
    let mut ss: String = String::new();
    let mut compress: bool = false;

    // First calculate if any 0 block must be collapsed and which one
    let mut i: usize = 0;
    while i != 16 {
        if locator.address[i] == 0 && locator.address[i + 1] == 0 {
            // 4.2.1 Shorten if it has already shortened the previous block
            if compress {
                actual_block_size += 1;
            } else {
                compress = true;
                actual_block_index = i;
                actual_block_size = 1;
            }
        } else {
            if compress {
                compress = false;
                // 4.2.3 Choose the largest block
                if actual_block_size > max_block_size {
                    max_block_index = actual_block_index;
                    max_block_size = actual_block_size;
                }
            }
        }
        i += 2;
    }

    // In case the last block is the one to compress
    if compress && actual_block_size > max_block_size {
        max_block_index = actual_block_index;
        max_block_size = actual_block_size;
    }

    // Use compress variable to know if there will be compression
    // 4.2.2 compress only if block size > 1
    compress = max_block_size >= 2;

    // Case that first block is compressed
    if compress && max_block_index == 0 {
        ss += ":";
    }

    i = 0;
    while i != 16 {
        // If it is after compress index and not all blocks has been compressed, compress it
        if compress && i >= max_block_index && max_block_size > 0 {
            // Reduce number of blocks to compress
            max_block_size -= 1;

            // Last block prints second :
            if max_block_size == 0 {
                ss += ":";
            }
            i += 2;
            continue;
        }

        // Stream the following block
        let field = ((locator.address[i] as u32) << 8) + locator.address[i + 1] as u32;
        if i != 14 {
            ss += &format!("{:x}", field);
            ss += ":";
        } else {
            ss += &format!("{:x}", field);
        }

        i += 2;
    }

    return ss;
}

pub fn isIPv4(address: &str) -> bool {
    lazy_static! {
        static ref IPv4_REGEX: Regex = Regex::new("^(?:(?:25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)\\.){3}(?:25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)$").unwrap();
    }
    IPv4_REGEX.is_match(address)
}

pub fn IPv6isCorrect(ipv6: &str) -> bool {
    // IPv6 addresses may have the interface ID added as in 'fe80::92f0:f536:e3cc:11c6%wlp2s0'
    let pos = match ipv6.find("%") {
        Some(p) => p,
        None => ipv6.len(),
    };

    let address = ipv6.substring(0, pos);

    /* An incorrect IPv6 format could be because:
     *  1. it has not ':' - bad format
     *  2. it has just one ':' - not enough info
     *  3. it has more than 8 ':' - too much info
     *  4. it has 8 ':' - it could only happen if it starts or ends with "::", if not is too much bytes
     *  5. it has more than one "::" - impossible to build the ip because unknown size of zero blocks
     *  6. it starts with ':' - it must be doble ("::") - bad format
     *  7. it ends with ':' - it must be doble ("::") - bad format
     **/
    let count = address.matches(":").count();

    // proper number of :
    if count < 2 || count > 8 {
        return false;
    }

    // only case of 8 : is with a :: at the beginning or end
    if count == 8
        && (address.chars().nth(0).unwrap() != ':'
            && address.chars().nth(address.len() - 1).unwrap() != ':')
    {
        return false;
    }

    // only one :: is allowed
    let _ind = match address.find("::") {
        Some(f) => match address[f + 1..].find("::") {
            Some(_s) => return false,
            _ => (),
        },
        _ => (),
    };

    // does not start with only one ':'
    if address.chars().nth(0).unwrap() == ':' && address.chars().nth(1).unwrap() != ':' {
        return false;
    }

    // does not end with only one ':'
    if address.chars().nth(address.len() - 1).unwrap() == ':'
        && address.chars().nth(address.len() - 2).unwrap() != ':'
    {
        return false;
    }

    // every number inside must not exceed ffff
    // do not accept any IPv6 with non valid characters
    lazy_static! {
        static ref IPv6_QUARTET_REGEX: Regex = Regex::new("^(?:[A-Fa-f0-9]){0,4}$").unwrap();
    }

    let split = address.split(":");
    for s in split {
        if s.len() == 0 {
            continue;
        }

        if !IPv6_QUARTET_REGEX.is_match(s) {
            return false;
        }
    }

    return true;
}

pub fn isIPv6(address: &str) -> bool {
    IPv6isCorrect(address)
}

pub fn setIPv4(locator: &mut Locator_t, address: &str) -> Result<(), AddrParseError> {
    let addr = Ipv4Addr::from_str(address);
    match addr {
        Ok(addr) => {
            locator.address[12] = addr.octets()[0];
            locator.address[13] = addr.octets()[1];
            locator.address[14] = addr.octets()[2];
            locator.address[15] = addr.octets()[3];
            Ok(())
        }
        Err(e) => return Err(e),
    }
}

pub fn setIPv6(locator: &mut Locator_t, address: &str) -> Result<(), AddrParseError> {
    let addr = Ipv6Addr::from_str(address);
    match addr {
        Ok(addr) => {
            locator.address = addr.octets();
            Ok(())
        }
        Err(e) => return Err(e),
    }
}

// Factory
pub fn createLocator(
    kindin: i32,
    address: &str,
    portin: u32,
    locator: &mut Locator_t,
) -> Result<(), AddrParseError> {
    locator.kind = kindin;
    locator.port = portin;
    LOCATOR_ADDRESS_INVALID!(locator.address);

    match kindin {
        LOCATOR_KIND_TCPv4 | LOCATOR_KIND_UDPv4 => {
            setIPv4(locator, address)?;
            Ok(())
        }
        LOCATOR_KIND_TCPv6 | LOCATOR_KIND_UDPv6 => {
            setIPv6(locator, address)?;
            Ok(())
        }
        _ => Ok(()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn IPv6isCorrect_test() {
        assert_eq!(true, IPv6isCorrect("fe80::92f0:f536:e3cc:11c6"));
        assert_eq!(
            true,
            IPv6isCorrect("2001:db8:3333:4444:5555:6666:7777:8888")
        );
        assert_eq!(
            true,
            IPv6isCorrect("2001:db8:3333:4444:CCCC:DDDD:EEEE:FFFF")
        );
        assert_eq!(true, IPv6isCorrect("::"));
        assert_eq!(true, IPv6isCorrect("2001:db8::"));
        assert_eq!(true, IPv6isCorrect("::1234:5678"));
        assert_eq!(true, IPv6isCorrect("2001:db8::1234:5678"));
        assert_eq!(
            true,
            IPv6isCorrect("2001:0db8:0001:0000:0000:0ab9:C0A8:0102")
        );

        assert_eq!(false, IPv6isCorrect("fe80:"));
    }
}
