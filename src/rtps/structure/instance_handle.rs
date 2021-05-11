use crate::rtps::structure::guid::GUID_t;

#[derive(Copy, Clone, Debug, PartialOrd, PartialEq, Ord, Eq)]
pub struct InstanceHandle_t {
    pub value: [u8; InstanceHandle_t::SIZE],
}

impl InstanceHandle_t {
    pub const SIZE: usize = 16;
    pub const c_InstanceHandle_Unknown: InstanceHandle_t = InstanceHandle_t {
        value: [0; InstanceHandle_t::SIZE],
    };

    pub fn new() -> Self {
        InstanceHandle_t {
            value: [0; InstanceHandle_t::SIZE],
        }
    }

    pub fn isDefined(&self) -> bool {
        for i in 0..16 {
            if self.value[i] != 0 {
                return true;
            }
        }
        return false;
    }

    pub fn as_guid_ref(&self) -> &GUID_t {
        let p = self as *const InstanceHandle_t;
        let g = p as *const GUID_t;
        unsafe {
            return &*g;
        }
    }

    #[inline(always)]
    pub fn iHandle2GUID(guid: &mut GUID_t, ihandle: &InstanceHandle_t) {
        for i in 0..16 {
            if i < 12 {
                guid.guidPrefix.value[i] = ihandle.value[i];
            } else {
                guid.entityId.value[i - 12] = ihandle.value[i];
            }
        }
    }
}

impl From<&GUID_t> for InstanceHandle_t {
    fn from(guid: &GUID_t) -> Self {
        let mut value: [u8; InstanceHandle_t::SIZE] = [0; InstanceHandle_t::SIZE];
        for i in 0..16 {
            if i < 12 {
                value[i] = guid.guidPrefix.value[i];
            } else {
                value[i] = guid.entityId.value[i - 12];
            }
        }
        InstanceHandle_t { value }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn as_guid_ref_test() {
        let mut instance_handle_t = InstanceHandle_t::new();
        instance_handle_t.value[0] = 1;
        instance_handle_t.value[13] = 1;
        let guid_ref = instance_handle_t.as_guid_ref();

        let mut guid: GUID_t = GUID_t::unknown();
        InstanceHandle_t::iHandle2GUID(&mut guid, &instance_handle_t);
        assert_eq!(*guid_ref == guid, true);
    }
}
