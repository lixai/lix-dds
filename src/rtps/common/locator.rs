use crate::base::net::lookup::*;
use crate::rtps::common::error::*;
use crate::rtps::utils::ip_locator::*;
use crate::LOCATOR_ADDRESS_INVALID;
use core::str::FromStr;
use std::string::ToString;
use substring::Substring;

/// Invalid locator kind
pub const LOCATOR_KIND_INVALID: i32 = -1;
/// Invalid locator port
pub const LOCATOR_PORT_INVALID: u32 = 0;
/// Reserved locator kind
pub const LOCATOR_KIND_RESERVED: i32 = 0;
/// UDP over IPv4 locator kind
pub const LOCATOR_KIND_UDPv4: i32 = 1;
/// UDP over IPv6 locator kind
pub const LOCATOR_KIND_UDPv6: i32 = 2;
/// TCP over IPv4 kind
pub const LOCATOR_KIND_TCPv4: i32 = 4;
/// TCP over IPv6 locator kind
pub const LOCATOR_KIND_TCPv6: i32 = 8;
/// Shared memory locator kind
pub const LOCATOR_KIND_SHM: i32 = 16;

/// Initialize locator with invalid values
#[allow(unused_macros)]
macro_rules! LOCATOR_INVALID {
    ($loc:expr) => {
        $loc.kind = LOCATOR_KIND_INVALID;
        $loc.port = LOCATOR_PORT_INVALID;
        LOCATOR_ADDRESS_INVALID!($loc.address);
    };
}

#[derive(Copy, Clone, Debug, PartialOrd, PartialEq, Ord, Eq)]
pub struct Locator_t {
    /**
     * @brief Specifies the locator type. Valid values are:
     *
     * LOCATOR_KIND_UDPv4
     *
     * LOCATOR_KIND_UDPv6
     *
     * LOCATOR_KIND_TCPv4
     *
     * LOCATOR_KIND_TCPv6
     *
     * LOCATOR_KIND_SHM
     */
    pub kind: i32,
    /// Network port
    pub port: u32,
    /// IP address
    pub address: [u8; 16],
}

impl Default for Locator_t {
    fn default() -> Self {
        Locator_t {
            kind: LOCATOR_KIND_UDPv4,
            port: 0,
            address: [0_u8; 16],
        }
    }
}

impl Locator_t {
    pub fn new(kind: i32, port: u32, address: [u8; 16]) -> Self {
        Locator_t {
            kind: kind,
            port: port,
            address: address,
        }
    }

    pub fn new_from_port(port: u32) -> Self {
        Locator_t {
            kind: LOCATOR_KIND_UDPv4,
            port: port,
            address: [0_u8; 16],
        }
    }

    pub fn new_from_kind_port(kind: i32, port: u32) -> Self {
        Locator_t {
            kind: kind,
            port: port,
            address: [0_u8; 16],
        }
    }

    /**
     * @brief Set the locator IP address using another locator.
     *
     * @param other Locator which IP address is used to set this locator IP address.
     * @return always true.
     */
    pub fn set_address(&mut self, other: &Self) -> bool {
        self.address = other.address;
        return true;
    }

    /**
     * @brief Getter for the locator IP address.
     *
     * @return IP address as octet pointer.
     */
    pub fn get_address(&mut self) -> &mut [u8; 16] {
        return &mut self.address;
    }

    /**
     * @brief Getter for a specific field of the locator IP address.
     *
     * @param field IP address element to be accessed.
     * @return Octet value for the specific IP address element.
     */
    pub fn get_address_by_field(&self, field: usize) -> u8 {
        return self.address[field];
    }

    /**
     * @brief Automatic setter for setting locator IP address to invalid address (0).
     */
    pub fn set_Invalid_Address(&mut self) {
        LOCATOR_ADDRESS_INVALID!(self.address);
    }
}

/**
 * @brief Insertion operator: serialize a locator
 *        The serialization format is kind:[address]:port
 *        \c kind must be one of the following:
 *            - UDPv4
 *            - UDPv6
 *            - TCPv4
 *            - TCPv6
 *            - SHM
 *        \c address IP address unless \c kind is SHM
 *        \c port number
 *
 * @param output Output stream where the serialized locator is appended.
 * @param loc Locator to be serialized/inserted.
 * @return \c std::ostream& Reference to the output stream with the serialized locator appended.
 */
impl ToString for Locator_t {
    fn to_string(&self) -> String {
        // Stream Locator kind
        let mut port = self.port;
        let kind = match self.kind {
            LOCATOR_KIND_TCPv4 => "TCPv4",
            LOCATOR_KIND_UDPv4 => "UDPv4",
            LOCATOR_KIND_TCPv6 => "TCPv6",
            LOCATOR_KIND_UDPv6 => "UDPv6",
            LOCATOR_KIND_SHM => "SHM",
            _ => {
                port = 0;
                "Invalid_locator"
            }
        };

        // Stream address
        let address = if self.kind == LOCATOR_KIND_UDPv4 || self.kind == LOCATOR_KIND_TCPv4 {
            toIPv4string(self)
        } else if self.kind == LOCATOR_KIND_UDPv6 || self.kind == LOCATOR_KIND_TCPv6 {
            toIPv6string(self)
        } else if self.kind == LOCATOR_KIND_SHM {
            if self.address[0] == 'M' as u8 {
                "M".to_string()
            } else {
                "_".to_string()
            }
        } else {
            "_".to_string()
        };

        format!("{}{}{}{}{}", kind, ":[", address, "]:", port)
    }
}

/**
 * @brief Extraction operator: deserialize a locator
 *        The deserialization format is kind:[address]:port
 *        \c kind must be one of the following:
 *            - UDPv4
 *            - UDPv6
 *            - TCPv4
 *            - TCPv6
 *            - SHM
 *        \c address must be either a name which can be resolved by DNS or the IP address unless \c kind is SHM
 *        \c port number
 *
 * @param input Input stream where the locator to be deserialized is located.
 * @param loc Locator where the deserialized locator is saved.
 * @return \c std::istream& Reference to the input stream after extracting the locator.
 */
impl FromStr for Locator_t {
    type Err = RtpsError;

    fn from_str(s: &str) -> Result<Self, RtpsError> {
        let mut loc = Locator_t::default();

        // Locator info
        //let mut kind: i32 = LOCATOR_KIND_INVALID;
        let kind: i32;
        let mut port: u32 = LOCATOR_PORT_INVALID;
        let mut address: String;

        // Check the locator kind
        let str_kind: &str;
        let str_kind_index: usize;
        if let Some(i) = s.find(":") {
            str_kind_index = i;
            str_kind = s.substring(0, str_kind_index);
        } else {
            return Err(RtpsError::new("Parse kind"));
        };

        match str_kind {
            "SHM" => kind = LOCATOR_KIND_SHM,
            "TCPv4" => kind = LOCATOR_KIND_TCPv4,
            "TCPv6" => kind = LOCATOR_KIND_TCPv6,
            "UDPv4" => kind = LOCATOR_KIND_UDPv4,
            "UDPv6" => kind = LOCATOR_KIND_UDPv6,
            _ => kind = LOCATOR_KIND_INVALID,
        }

        // Get address in strings
        let str_address: &str;
        if let Some(i) = s.find("]") {
            // Ignore chars :[
            str_address = s.substring(str_kind_index + 2, i);
        } else {
            return Err(RtpsError::new("Get address"));
        };
        address = str_address.to_string();

        // check if this is a valid IPv4 or IPv6 and call DNS if not
        if (kind == LOCATOR_KIND_UDPv4 || kind == LOCATOR_KIND_TCPv4) && !isIPv4(str_address) {
            match lookup_ipv4(str_address) {
                Ok(i) => {
                    address = i;
                }
                Err(_w) => {
                    loc.kind = LOCATOR_KIND_INVALID;
                    return Err(RtpsError::new("IPv4 lookup host"));
                }
            }
        }

        if (kind == LOCATOR_KIND_UDPv6 || kind == LOCATOR_KIND_TCPv6) && !isIPv6(str_address) {
            match lookup_ipv6(str_address) {
                Ok(i) => {
                    address = i;
                }
                Err(_w) => {
                    loc.kind = LOCATOR_KIND_INVALID;
                    return Err(RtpsError::new("IPv6 lookup host"));
                }
            }
        }

        // Get port
        let str_port: &str;
        if let Some(i) = s.find("]:") {
            str_port = s.substring(i + 2, s.len());
            port = str_port.parse()?;
        }

        loc.kind = kind;
        match createLocator(kind, &address, port, &mut loc) {
            Ok(_) => Ok(loc),
            Err(_) => {
                return Err(RtpsError::new("Create locator"));
            }
        }
    }
}

/**
 * @brief Auxiliary method to check that IP address is not invalid (0).
 *
 * @param loc Locator which IP address is going to be checked.
 * @return true if IP address is defined (not 0).
 * @return false otherwise.
 */
#[inline]
fn IsAddressDefined(loc: &Locator_t) -> bool {
    if loc.kind == LOCATOR_KIND_UDPv4 || loc.kind == LOCATOR_KIND_TCPv4
    // WAN addr in TCPv4 is optional, isn't?
    {
        for i in 12..16 {
            if loc.address[i] != 0 {
                return true;
            }
        }
    } else if loc.kind == LOCATOR_KIND_UDPv6 || loc.kind == LOCATOR_KIND_TCPv6 {
        for i in 0..16 {
            if loc.address[i] != 0 {
                return true;
            }
        }
    }
    return false;
}

/**
 * @brief Auxiliary method to check that locator kind is not LOCATOR_KIND_INVALID (-1).
 *
 * @param loc Locator to be checked.
 * @return true if the locator kind is not LOCATOR_KIND_INVALID.
 * @return false otherwise.
 */
#[inline]
fn IsLocatorValid(loc: &Locator_t) -> bool {
    return 0 <= loc.kind;
}

type LocatorList = std::vec::Vec<Locator_t>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn macro_test() {
        let address = [1_u8; 16];
        let inited_address = [0_u8; 16];
        let mut locator = Locator_t::new(0, 1, address);

        LOCATOR_INVALID!(locator);
        assert_eq!(LOCATOR_KIND_INVALID, locator.kind);
        assert_eq!(LOCATOR_PORT_INVALID, locator.port);
        assert_eq!(inited_address, locator.address);
    }

    #[test]
    fn contructor_test() {
        let inited_address = [0_u8; 16];
        let locator = Locator_t::default();
        assert_eq!(LOCATOR_KIND_UDPv4, locator.kind);
        assert_eq!(0, locator.port);
        assert_eq!(inited_address, locator.address);
    }

    #[test]
    fn address_test() {
        let address1 = [1_u8; 16];
        let locator1 = Locator_t::new(LOCATOR_KIND_UDPv6, 2, address1);

        let mut locator2 = Locator_t::default();
        locator2.set_address(&locator1);

        let address2 = locator2.get_address();
        assert_eq!(&address1, address2);
    }

    #[test]
    fn IsAddressDefined_test() {
        let mut locator = Locator_t::default();
        assert_eq!(IsAddressDefined(&locator), false);

        locator.address = [1_u8; 16];
        assert_eq!(IsAddressDefined(&locator), true);

        locator.kind = LOCATOR_KIND_UDPv6;
        assert_eq!(IsAddressDefined(&locator), true);

        locator.kind = LOCATOR_KIND_RESERVED;
        assert_eq!(IsAddressDefined(&locator), false);
    }

    #[test]
    fn IsLocatorValid_test() {
        let mut locator = Locator_t::new(LOCATOR_KIND_INVALID, 0, [0_u8; 16]);
        assert_eq!(IsLocatorValid(&locator), false);

        locator.kind = LOCATOR_KIND_UDPv4;
        assert_eq!(IsLocatorValid(&locator), true);

        locator.kind = LOCATOR_KIND_RESERVED;
        assert_eq!(IsLocatorValid(&locator), true);
    }

    #[test]
    fn to_string_test() {
        let mut locator = Locator_t::new(LOCATOR_KIND_INVALID, 1, [0xFF_u8; 16]);
        assert_eq!("Invalid_locator:[_]:0", locator.to_string());

        locator.kind = LOCATOR_KIND_RESERVED;
        assert_eq!("Invalid_locator:[_]:0", locator.to_string());

        locator.kind = LOCATOR_KIND_UDPv4;
        assert_eq!("UDPv4:[255.255.255.255]:1", locator.to_string());

        locator.kind = LOCATOR_KIND_UDPv6;
        assert_eq!(
            "UDPv6:[ffff:ffff:ffff:ffff:ffff:ffff:ffff:ffff]:1",
            locator.to_string()
        );

        locator.kind = LOCATOR_KIND_TCPv4;
        assert_eq!("TCPv4:[255.255.255.255]:1", locator.to_string());

        locator.kind = LOCATOR_KIND_TCPv6;
        assert_eq!(
            "TCPv6:[ffff:ffff:ffff:ffff:ffff:ffff:ffff:ffff]:1",
            locator.to_string()
        );

        locator.kind = LOCATOR_KIND_SHM;
        assert_eq!("SHM:[_]:1", locator.to_string());
    }

    #[test]
    fn from_str_test() {
        // kind:[address]:port
        let mut loc1 = Locator_t::new(LOCATOR_KIND_UDPv4, 1, [0xFF_u8; 16]);
        let mut loc2 = Locator_t::from_str(&loc1.to_string()).unwrap();
        assert_eq!(loc1.to_string(), loc2.to_string());

        loc1 = Locator_t::new(LOCATOR_KIND_TCPv4, 1, [0xFF_u8; 16]);
        loc2 = Locator_t::from_str(&loc1.to_string()).unwrap();
        assert_eq!(loc1.to_string(), loc2.to_string());

        loc1 = Locator_t::new(LOCATOR_KIND_UDPv6, 1, [0xFF_u8; 16]);
        loc2 = Locator_t::from_str(&loc1.to_string()).unwrap();
        assert_eq!(loc1.to_string(), loc2.to_string());

        loc1 = Locator_t::new(LOCATOR_KIND_TCPv6, 1, [0xFF_u8; 16]);
        loc2 = Locator_t::from_str(&loc1.to_string()).unwrap();
        assert_eq!(loc1.to_string(), loc2.to_string());

        loc1 = Locator_t::new(LOCATOR_KIND_SHM, 1, [0xFF_u8; 16]);
        loc2 = Locator_t::from_str(&loc1.to_string()).unwrap();
        assert_eq!(loc1.to_string(), loc2.to_string());

        loc1 = Locator_t::new(LOCATOR_KIND_INVALID, 1, [0xFF_u8; 16]);
        loc2 = Locator_t::from_str(&loc1.to_string()).unwrap();
        assert_eq!(loc1.to_string(), loc2.to_string());

        loc2 = Locator_t::from_str("UDPv4:[localhost]:2").unwrap();
        assert_eq!("UDPv4:[127.0.0.1]:2", loc2.to_string());

        loc2 = Locator_t::from_str("TCPv4:[localhost]:2").unwrap();
        assert_eq!("TCPv4:[127.0.0.1]:2", loc2.to_string());

        loc2 = Locator_t::from_str("UDPv6:[localhost]:2").unwrap();
        assert_eq!("UDPv6:[::1]:2", loc2.to_string());

        loc2 = Locator_t::from_str("TCPv6:[localhost]:2").unwrap();
        assert_eq!("TCPv6:[::1]:2", loc2.to_string());
    }
}
