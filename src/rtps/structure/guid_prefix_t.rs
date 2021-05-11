#[derive(Copy, Clone, Debug, PartialOrd, PartialEq, Ord, Eq)]
pub struct GuidPrefix_t {
    pub value: [u8; GuidPrefix_t::SIZE],
}

impl GuidPrefix_t {
    pub const SIZE: usize = 12;
}

impl GuidPrefix_t {
    pub const c_GuidPrefix_Unknown: GuidPrefix_t = GuidPrefix_t::unknown();
}

impl GuidPrefix_t {
    pub const fn unknown() -> Self {
        GuidPrefix_t {
            value: [0x00; GuidPrefix_t::SIZE],
        }
    }
}

impl Default for GuidPrefix_t {
    fn default() -> Self {
        GuidPrefix_t::unknown()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unknown_test() {
        assert_eq!(GuidPrefix_t::unknown().value, [0x00; GuidPrefix_t::SIZE]);
    }

    #[test]
    fn default_test() {
        assert_eq!(GuidPrefix_t::default().value, [0x00; GuidPrefix_t::SIZE]);
    }

    #[test]
    fn not_equals_test() {
        let v1 = GuidPrefix_t {
            value: [
                0x00, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88, 0x99, 0xAA, 0xBB,
            ],
        };

        let v2 = GuidPrefix_t::default();

        assert!(v1 != v2);
    }

    #[test]
    fn equals_test() {
        let v1 = GuidPrefix_t {
            value: [
                0x00, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88, 0x99, 0xAA, 0xBB,
            ],
        };

        let v2 = GuidPrefix_t {
            value: [
                0x00, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88, 0x99, 0xAA, 0xBB,
            ],
        };

        assert!(v1 == v2);
    }

    #[test]
    fn greater_than() {
        let v1 = GuidPrefix_t {
            value: [
                0x00, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88, 0x99, 0xAA, 0xBB,
            ],
        };

        let v2 = GuidPrefix_t::default();

        assert!(v1 >= v2);
        assert!(v2 <= v1);
    }
}
