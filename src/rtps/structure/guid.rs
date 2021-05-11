use crate::rtps::structure::guid_prefix_t::GuidPrefix_t;
use crate::rtps::structure::entity_id_t::EntityId_t;

#[derive(Copy, Clone, Debug, PartialOrd, PartialEq, Ord, Eq)]
pub struct GUID_t {
    pub guidPrefix: GuidPrefix_t,
    pub entityId: EntityId_t,
}

impl GUID_t {
    pub const c_Guid_Unknown: GUID_t = GUID_t {
        guidPrefix: GuidPrefix_t::c_GuidPrefix_Unknown,
        entityId: EntityId_t::c_EntityId_Unknown
    };
}

impl GUID_t {
    pub fn unknown() -> Self {
        GUID_t {
            guidPrefix: GuidPrefix_t::unknown(),
            entityId: EntityId_t::unknown()
        }
    }

    pub fn is_on_same_host_as(&self, other_guid: &GUID_t) -> bool {
        &self.guidPrefix.value[..4] == &other_guid.guidPrefix.value[..4]
    }

    pub fn is_on_same_process_as(&self, other_guid: &GUID_t) -> bool {
        &self.guidPrefix.value[..8] == &other_guid.guidPrefix.value[..8]
    }

    pub fn is_builtin(&self) -> bool {
        self.entityId.value[3] >= 0xC0u8
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn partial_ord_test() {
        let id1 = GUID_t::unknown();
        let id2 = GUID_t::c_Guid_Unknown;
        assert_eq!(id1 == id2, true);
        assert_eq!(id1 >= id2, true);
        assert_eq!(id1 <= id2, true);

        let id3 = GUID_t {
            guidPrefix: GuidPrefix_t::unknown(),
            entityId: EntityId_t::unknown()
        };
        let mut id4 = GUID_t {
            guidPrefix: GuidPrefix_t::unknown(),
            entityId: EntityId_t::unknown()
        };
        id4.entityId = EntityId_t::c_EntityId_SPDPReader;
        assert_eq!(id3 < id4, true);
        assert_eq!(id3 <= id4, true);

        let mut id5 = GUID_t {
            guidPrefix: GuidPrefix_t::unknown(),
            entityId: EntityId_t::unknown()
        };
        id5.guidPrefix.value = [0x00, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88, 0x99, 0xAA, 0xBB,];
        assert_eq!(id3 < id5, true);
        assert_eq!(id3 <= id5, true);

        let mut id6 = GUID_t::unknown();
        let mut id7 = GUID_t::unknown();
        id6.guidPrefix.value = [0x00, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88, 0x99, 0xAA, 0xBB];
        id7.entityId = EntityId_t::c_EntityId_SPDPReader;
        assert_eq!(id6 > id7, true);
        assert_eq!(id6 >= id7, true);
    }

    #[test]
    fn is_on_same_host_and_process_test() {
        let mut id1 = GUID_t {
            guidPrefix: GuidPrefix_t::unknown(),
            entityId: EntityId_t::unknown()
        };
        id1.guidPrefix.value = [0x00, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88, 0x99, 0xAA, 0xBB,];

        let mut id2 = GUID_t {
            guidPrefix: GuidPrefix_t::unknown(),
            entityId: EntityId_t::unknown()
        };
        id2.guidPrefix.value = [0x00, 0x11, 0x22, 0x33, 0x44, 0x56, 0x66, 0x77, 0x88, 0x99, 0xAA, 0xBB,];

        let mut id3 = GUID_t {
            guidPrefix: GuidPrefix_t::unknown(),
            entityId: EntityId_t::unknown()
        };
        id3.guidPrefix.value = [0x00, 0x11, 0x22, 0x33, 0x44, 0x56, 0x66, 0x77, 0x89, 0x99, 0xAA, 0xBB,];

        assert_eq!(id1.is_on_same_host_as(&id2), true);
        assert_eq!(id2.is_on_same_process_as(&id3), true);
        assert_eq!(id1.is_on_same_process_as(&id3), false);
    }

    #[test]
    fn is_builtin_test() {
        let mut id1 = GUID_t {
            guidPrefix: GuidPrefix_t::unknown(),
            entityId: EntityId_t::unknown()
        };

        id1.entityId.value[3] = 0xc0;
        assert_eq!(id1.is_builtin(), true);
        id1.entityId.value[3] = 0xc1;
        assert_eq!(id1.is_builtin(), true);
        id1.entityId.value[3] = 0xbf;
        assert_eq!(id1.is_builtin(), false);
    }
}
