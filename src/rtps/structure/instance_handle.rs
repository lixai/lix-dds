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
}

/**
 * Convert InstanceHandle_t to GUID
 * @param guid GUID to store the results
 * @param ihandle InstanceHandle_t to copy
 */
trait InstanceHandle_t_to_GUID_1 {
    fn iHandle2GUID(guid: &mut GUID_t, ihandle:&InstanceHandle_t);
}

/**
 * Convert InstanceHandle_t to GUID
 * @param ihandle InstanceHandle_t to store the results
 * @return GUID_t
 */
trait InstanceHandle_t_to_GUID_2 {
    fn iHandle2GUID(ihandle: &InstanceHandle_t) -> GUID_t;
}

impl InstanceHandle_t_to_GUID_1 for InstanceHandle_t {
    fn iHandle2GUID(guid: &mut GUID_t, ihandle: &InstanceHandle_t) {
        for i in 0..16 {
            if i < 12 {
                guid.guidPrefix.value[i] = ihandle.value[i];
            } else {
                guid.entityId.value[i - 12] = ihandle.value[i];
            }
        }
    }
}

impl InstanceHandle_t_to_GUID_2 for InstanceHandle_t {
    fn iHandle2GUID(ihandle: &InstanceHandle_t) -> GUID_t {
        let mut guid: GUID_t = GUID_t { guidPrefix: Default::default(), entityId: Default::default() };
        for i in 0..16 {
            if i < 12 {
                guid.guidPrefix.value[i] = ihandle.value[i];
            } else {
                guid.entityId.value[i - 12] = ihandle.value[i];
            }
        }
        return guid;
    }
}

/*impl PartialOrd for InstanceHandle_t {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.value == other.value {
            return Some(Ordering::Equal);
        } else if self.value >= other.value {
            return Some(Ordering::Greater);
        } else if self.value <= other.value {
            return Some(Ordering::Less);
        } else {
            None
        }
    }
}*/

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn guid_instance_handle_t_tests() {
        let mut instance_handle_t = InstanceHandle_t::new();
        instance_handle_t.value[0] = 1;
        instance_handle_t.value[13] = 1;
        let guid_ref = instance_handle_t.as_guid_ref();

        let mut guid: GUID_t = GUID_t::unknown();
        <InstanceHandle_t as InstanceHandle_t_to_GUID_1>::iHandle2GUID(&mut guid, &instance_handle_t);
        assert_eq!(*guid_ref == guid, true);

        guid = <InstanceHandle_t as InstanceHandle_t_to_GUID_2>::iHandle2GUID(&instance_handle_t);
        assert_eq!(*guid_ref == guid, true);
    }

    #[test]
    fn comparison_tests() {
        let mut v1 = InstanceHandle_t::c_InstanceHandle_Unknown;
        let mut v2 = InstanceHandle_t::new();

        assert!(v1 == v2);

        v1.value[0] = 1;
        assert!(v1 > v2);
        assert!(v1 >= v2);

        v2.value[0] = 1;
        assert!(v1 >= v2);
        assert!(v1 <= v2);

        v2.value[1] = 1;
        assert!(v1 < v2);
        assert!(v1 <= v2);
    }
}
