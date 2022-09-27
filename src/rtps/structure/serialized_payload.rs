const CDR_BE: u16 = 0x0000;
const CDR_LE: u16 = 0x0001;
const PL_CDR_BE: u16 = 0x0002;
const PL_CDR_LE: u16 = 0x0003;

#[cfg(target_endian = "little")]
const DEFAULT_ENCAPSULATION: u16 = CDR_LE;
#[cfg(target_endian = "little")]
const PL_DEFAULT_ENCAPSULATION: u16 = PL_CDR_LE;

#[cfg(target_endian = "big")]
const DEFAULT_ENCAPSULATION: u16 = CDR_LE;
#[cfg(target_endian = "big")]
const PL_DEFAULT_ENCAPSULATION: u16 = PL_CDR_BE;

#[derive(Debug)]
pub struct SerializedPayload_t {
    // Encapsulation of the data as suggested in the RTPS 2.1 specification chapter 10.
    encapsulation: u16,
    // Actual length of the data
    length: usize,
    // Pointer to the data.
    data: Vec<u8>,
    // Maximum size of the payload
    max_size: usize,
    // Position when reading
    pos: usize,
}

impl Default for SerializedPayload_t {
    fn default() -> Self {
        SerializedPayload_t {
            encapsulation: CDR_BE,
            length: 0,
            data: vec![],
            max_size: 0,
            pos: 0,
        }
    }
}

impl PartialEq for SerializedPayload_t {
    fn eq(&self, other: &Self) -> bool {
        if self.encapsulation != other.encapsulation || self.length != other.length {
            return false;
        }

        for n in 0..self.length {
            if self.data[n] != other.data[n] {
                return false;
            }
        }
        return true;
    }
}

impl SerializedPayload_t {
    //!Size in bytes of the representation header as specified in the RTPS 2.3 specification chapter 10.
    pub const representation_header_size: usize = 4;

    /*
     * Copy another structure (including allocating new space for the data.)
     * @param[in] serData Pointer to the structure to copy
     * @param with_limit if true, the function will fail when providing a payload too big
     * @return True if correct
     */
    pub fn copy(&mut self, serData: &SerializedPayload_t, with_limit: bool) -> bool {
        self.length = serData.length;

        if serData.length > self.max_size {
            if with_limit {
                println!("with_limit false");
                return false;
            } else {
                self.reserve(serData.length);
            }
        }
        self.encapsulation = serData.encapsulation;
        if self.length == 0 {
            return true;
        }
        self.data.resize(self.length, 0);
        let t = &serData.data[0..self.length];
        self.data.copy_from_slice(t);
        return true;
    }

    /*
     * Allocate new space for fragmented data
     * @param[in] serData Pointer to the structure to copy
     * @return True if correct
     */
    pub fn reserve_fragmented(&mut self, serData: &SerializedPayload_t) -> bool {
        self.length = serData.length;
        self.max_size = serData.length;
        self.encapsulation = serData.encapsulation;
        self.data.resize(self.length, 0);
        return true;
    }

    // Empty the payload
    pub fn empty(&mut self) {
        self.length = 0;
        self.encapsulation = CDR_BE;
        self.max_size = 0;
        if !self.data.is_empty() {
            self.data.clear();
        }
    }

    pub fn reserve(&mut self, new_size: usize) {
        if new_size <= self.max_size {
            return;
        }

        self.data.resize(new_size, 0);
        self.max_size = new_size;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn eq_operator_test() {
        let mut sp1 = SerializedPayload_t::default();
        let mut sp2 = SerializedPayload_t::default();
        assert_eq!(sp1, sp2);

        sp1.reserve(8);
        assert_eq!(sp1, sp2);

        sp1.length = 1;
        sp1.data[0] = 1;

        assert_ne!(sp1, sp2);

        assert!(!sp2.copy(&sp1, true));
        assert!(sp2.copy(&sp1, false));

        assert_eq!(sp1, sp2);
    }
}
