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

pub struct SerializedPayload_t {
    // Encapsulation of the data as suggested in the RTPS 2.1 specification chapter 10.
    encapsulation: u16,
    // Actual length of the data
    length: u32,
    // Pointer to the data.
    data: Option<Box<u8>>,
    // Maximum size of the payload
    max_size: u32,
    // Position when reading
    pos: u32,
}

impl Default for SerializedPayload_t {
    fn default() -> Self {
        SerializedPayload_t {
            encapsulation: CDR_BE,
            length: 0,
            data: None,
            max_size: 0,
            pos: 0,
        }
    }
}

impl SerializedPayload_t {
    //!Size in bytes of the representation header as specified in the RTPS 2.3 specification chapter 10.
    pub const representation_header_size: usize = 4;

    pub fn reserve(&mut self, new_size: u32) {
        if new_size <= self.max_size {
            return;
        }

        match self.data {
            None => {
                self.data = Box::new();
                data = (u8*)calloc(new_size, sizeof(octet));
                if (!data) {
                    throw std::bad_alloc();
                }
            }

            Some(x) => {

            }
        }

        if data == nullptr {
            data = (octet*)calloc(new_size, sizeof(octet));
            if (!data)
            {
                throw std::bad_alloc();
            }
        }
        else
        {
            void* old_data = data;
            data = (octet*)realloc(data, new_size);
            if (!data)
            {
                free(old_data);
                throw std::bad_alloc();
            }
            memset(data + max_size, 0, (new_size - max_size) * sizeof(octet));
        }
        max_size = new_size;
    }
}