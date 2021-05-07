use byteorder::ByteOrder;

#[derive(Copy, Clone, Debug, PartialOrd, PartialEq, Ord, Eq)]
pub struct EntityId_t {
    pub value: [u8; EntityId_t::SIZE],
}

impl EntityId_t {
    pub const SIZE: usize = 4;

    pub const ENTITYID_UNKNOWN: u32 = 0x00000000;
    pub const ENTITYID_RTPSParticipant: u32 = 0x000001c1;
    pub const ENTITYID_SEDP_BUILTIN_TOPIC_WRITER: u32 = 0x000002c2;
    pub const ENTITYID_SEDP_BUILTIN_TOPIC_READER: u32 = 0x000002c7;
    pub const ENTITYID_SEDP_BUILTIN_PUBLICATIONS_WRITER: u32 = 0x000003c2;
    pub const ENTITYID_SEDP_BUILTIN_PUBLICATIONS_READER: u32 = 0x000003c7;
    pub const ENTITYID_SEDP_BUILTIN_SUBSCRIPTIONS_WRITER: u32 = 0x000004c2;
    pub const ENTITYID_SEDP_BUILTIN_SUBSCRIPTIONS_READER: u32 = 0x000004c7;
    pub const ENTITYID_SPDP_BUILTIN_RTPSParticipant_WRITER: u32 = 0x000100c2;
    pub const ENTITYID_SPDP_BUILTIN_RTPSParticipant_READER: u32 = 0x000100c7;
    pub const ENTITYID_P2P_BUILTIN_RTPSParticipant_MESSAGE_WRITER: u32 = 0x000200C2;
    pub const ENTITYID_P2P_BUILTIN_RTPSParticipant_MESSAGE_READER: u32 = 0x000200C7;
    pub const ENTITYID_P2P_BUILTIN_PARTICIPANT_STATELESS_WRITER: u32 = 0x000201C3;
    pub const ENTITYID_P2P_BUILTIN_PARTICIPANT_STATELESS_READER: u32 = 0x000201C4;

    pub const ENTITYID_TL_SVC_REQ_WRITER: u32 = 0x000300C3;
    pub const ENTITYID_TL_SVC_REQ_READER: u32 = 0x000300C4;
    pub const ENTITYID_TL_SVC_REPLY_WRITER: u32 = 0x000301C3;
    pub const ENTITYID_TL_SVC_REPLY_READER: u32 = 0x000301C4;

    // HAVE_SECURITY
    pub const ENTITYID_SEDP_BUILTIN_PUBLICATIONS_SECURE_WRITER: u32 = 0xff0003c2;
    pub const ENTITYID_SEDP_BUILTIN_PUBLICATIONS_SECURE_READER: u32 = 0xff0003c7;
    pub const ENTITYID_SEDP_BUILTIN_SUBSCRIPTIONS_SECURE_WRITER: u32 = 0xff0004c2;
    pub const ENTITYID_SEDP_BUILTIN_SUBSCRIPTIONS_SECURE_READER: u32 = 0xff0004c7;
    pub const ENTITYID_P2P_BUILTIN_PARTICIPANT_MESSAGE_SECURE_WRITER: u32 = 0xff0200c2;
    pub const ENTITYID_P2P_BUILTIN_PARTICIPANT_MESSAGE_SECURE_READER: u32 = 0xff0200c7;
    pub const ENTITYID_P2P_BUILTIN_PARTICIPANT_VOLATILE_MESSAGE_SECURE_WRITER: u32 = 0xff0202C3;
    pub const ENTITYID_P2P_BUILTIN_PARTICIPANT_VOLATILE_MESSAGE_SECURE_READER: u32 = 0xff0202C4;
    pub const ENTITYID_SPDP_RELIABLE_BUILTIN_PARTICIPANT_SECURE_WRITER: u32 = 0xff0101c2;
    pub const ENTITYID_SPDP_RELIABLE_BUILTIN_PARTICIPANT_SECURE_READER: u32 = 0xff0101c7;
    // end HAVE_SECURITY

    pub const ENTITYID_DS_SERVER_VIRTUAL_WRITER: u32 = 0x00030073;
    pub const ENTITYID_DS_SERVER_VIRTUAL_READER: u32 = 0x00030074;
}

impl EntityId_t {
    pub const c_EntityId_Unknown: EntityId_t = EntityId_t {
        value: [0x00, 0x00, 0x00, 0x00],
    };

    pub const c_EntityId_SPDPReader: EntityId_t = EntityId_t {
        value: [0x00, 0x01, 0x00, 0xc7],
    };
    pub const c_EntityId_SPDPWriter: EntityId_t = EntityId_t {
        value: [0x00, 0x01, 0x00, 0xc2],
    };

    pub const c_EntityId_SEDPPubWriter: EntityId_t = EntityId_t {
        value: [0x00, 0x00, 0x03, 0xc2],
    };
    pub const c_EntityId_SEDPPubReader: EntityId_t = EntityId_t {
        value: [0x00, 0x00, 0x03, 0xc7],
    };
    pub const c_EntityId_SEDPSubWriter: EntityId_t = EntityId_t {
        value: [0x00, 0x00, 0x04, 0xc2],
    };
    pub const c_EntityId_SEDPSubReader: EntityId_t = EntityId_t {
        value: [0x00, 0x00, 0x04, 0xc7],
    };

    pub const c_EntityId_RTPSParticipant: EntityId_t = EntityId_t {
        value: [0x00, 0x00, 0x01, 0xc1],
    };

    pub const c_EntityId_WriterLiveliness: EntityId_t = EntityId_t {
        value: [0x00, 0x02, 0x00, 0xC2],
    };
    pub const c_EntityId_ReaderLiveliness: EntityId_t = EntityId_t {
        value: [0x00, 0x02, 0x00, 0xC7],
    };

    pub const participant_stateless_message_writer_entity_id: EntityId_t = EntityId_t {
        value: [0x00, 0x02, 0x01, 0xC3],
    };
    pub const participant_stateless_message_reader_entity_id: EntityId_t = EntityId_t {
        value: [0x00, 0x02, 0x01, 0xC4],
    };

    pub const c_EntityId_TypeLookup_request_writer: EntityId_t = EntityId_t {
        value: [0x00, 0x03, 0x00, 0xC3],
    };
    pub const c_EntityId_TypeLookup_request_reader: EntityId_t = EntityId_t {
        value: [0x00, 0x03, 0x00, 0xC4],
    };
    pub const c_EntityId_TypeLookup_reply_writer: EntityId_t = EntityId_t {
        value: [0x00, 0x03, 0x01, 0xC3],
    };
    pub const c_EntityId_TypeLookup_reply_reader: EntityId_t = EntityId_t {
        value: [0x00, 0x03, 0x01, 0xC4],
    };

    // HAVE_SECURITY
    pub const sedp_builtin_publications_secure_writer: EntityId_t = EntityId_t {
        value: [0xff, 0x00, 0x03, 0xc2],
    };
    pub const sedp_builtin_publications_secure_reader: EntityId_t = EntityId_t {
        value: [0xff, 0x00, 0x03, 0xc7],
    };
    pub const sedp_builtin_subscriptions_secure_writer: EntityId_t = EntityId_t {
        value: [0xff, 0x00, 0x04, 0xc2],
    };
    pub const sedp_builtin_subscriptions_secure_reader: EntityId_t = EntityId_t {
        value: [0xff, 0x00, 0x04, 0xc7],
    };

    pub const participant_volatile_message_secure_writer_entity_id: EntityId_t = EntityId_t {
        value: [0xff, 0x02, 0x02, 0xC3],
    };
    pub const participant_volatile_message_secure_reader_entity_id: EntityId_t = EntityId_t {
        value: [0xff, 0x02, 0x02, 0xC4],
    };

    pub const c_EntityId_WriterLivelinessSecure: EntityId_t = EntityId_t {
        value: [0xff, 0x02, 0x00, 0xc2],
    };
    pub const c_EntityId_ReaderLivelinessSecure: EntityId_t = EntityId_t {
        value: [0xff, 0x02, 0x00, 0xc7],
    };
    // end HAVE_SECURITY

    pub const ds_server_virtual_writer: EntityId_t = EntityId_t {
        value: [0x00, 0x03, 0x00, 0x73],
    };
    pub const ds_server_virtual_reader: EntityId_t = EntityId_t {
        value: [0x00, 0x03, 0x00, 0x74],
    };
}

impl EntityId_t {
    pub fn unknown() -> EntityId_t {
        EntityId_t::c_EntityId_Unknown
    }

    #[cfg(target_endian = "little")]
    pub fn reverse(value: &mut [u8; EntityId_t::SIZE]) {
        let mut oaux = value[3];
        value[3] = value[0];
        value[0] = oaux;
        oaux = value[2];
        value[2] = value[1];
        value[1] = oaux;
    }

    #[cfg(target_endian = "little")]
    pub fn new(id: u32) -> EntityId_t {
        let index = 0;
        let mut value: [u8; EntityId_t::SIZE] = [0, 0, 0, 0];
        byteorder::NativeEndian::write_u32(&mut value[index..index + 4], id);
        EntityId_t::reverse(&mut value);
        EntityId_t { value }
    }

    #[cfg(target_endian = "big")]
    pub fn new(id: u32) -> EntityId_t {
        let index = 0;
        let mut value: [u8; EntityId_t::SIZE] = [0, 0, 0, 0];
        byteorder::NativeEndian::write_u32(&mut value[index..index + 4], id);
        EntityId_t { value }
    }

    pub fn hash(k: &EntityId_t) -> usize {
        (k.value[0] as usize) << 16 | (k.value[1] as usize) << 8 | (k.value[2] as usize)
    }
}

impl Default for EntityId_t {
    fn default() -> EntityId_t {
        EntityId_t::c_EntityId_Unknown.clone()
    }
}

impl PartialEq<u32> for EntityId_t {
    fn eq(&self, other: &u32) -> bool {
        let other_id = EntityId_t::new(*other);
        self == &other_id
    }
}

impl PartialEq<EntityId_t> for u32 {
    fn eq(&self, other: &EntityId_t) -> bool {
        let self_id = EntityId_t::new(*self);
        self_id == *other
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn entity_id_test() {
        assert_eq!(
            EntityId_t::new(EntityId_t::ENTITYID_UNKNOWN).value,
            EntityId_t::c_EntityId_Unknown.value
        );

        assert_eq!(
            EntityId_t::new(EntityId_t::ENTITYID_RTPSParticipant).value,
            EntityId_t::c_EntityId_RTPSParticipant.value
        );

        assert_eq!(
            EntityId_t::new(EntityId_t::ENTITYID_SEDP_BUILTIN_TOPIC_WRITER).value,
            [0x00u8, 0x00u8, 0x02u8, 0xc2u8]
        );

        assert_eq!(
            EntityId_t::new(EntityId_t::ENTITYID_SEDP_BUILTIN_TOPIC_READER).value,
            [0x00u8, 0x00u8, 0x02u8, 0xc7u8]
        );

        assert_eq!(
            EntityId_t::new(EntityId_t::ENTITYID_SEDP_BUILTIN_PUBLICATIONS_WRITER).value,
            EntityId_t::c_EntityId_SEDPPubWriter.value
        );

        assert_eq!(
            EntityId_t::new(EntityId_t::ENTITYID_SEDP_BUILTIN_PUBLICATIONS_READER).value,
            EntityId_t::c_EntityId_SEDPPubReader.value
        );

        assert_eq!(
            EntityId_t::new(EntityId_t::ENTITYID_SEDP_BUILTIN_SUBSCRIPTIONS_WRITER).value,
            EntityId_t::c_EntityId_SEDPSubWriter.value
        );

        assert_eq!(
            EntityId_t::new(EntityId_t::ENTITYID_SEDP_BUILTIN_SUBSCRIPTIONS_READER).value,
            EntityId_t::c_EntityId_SEDPSubReader.value
        );

        assert_eq!(
            EntityId_t::new(EntityId_t::ENTITYID_SPDP_BUILTIN_RTPSParticipant_WRITER).value,
            EntityId_t::c_EntityId_SPDPWriter.value
        );

        assert_eq!(
            EntityId_t::new(EntityId_t::ENTITYID_SPDP_BUILTIN_RTPSParticipant_READER).value,
            EntityId_t::c_EntityId_SPDPReader.value
        );

        assert_eq!(
            EntityId_t::new(EntityId_t::ENTITYID_P2P_BUILTIN_RTPSParticipant_MESSAGE_WRITER).value,
            EntityId_t::c_EntityId_WriterLiveliness.value
        );

        assert_eq!(
            EntityId_t::new(EntityId_t::ENTITYID_P2P_BUILTIN_RTPSParticipant_MESSAGE_READER).value,
            EntityId_t::c_EntityId_ReaderLiveliness.value
        );

        assert_eq!(
            EntityId_t::new(EntityId_t::ENTITYID_P2P_BUILTIN_PARTICIPANT_STATELESS_WRITER).value,
            EntityId_t::participant_stateless_message_writer_entity_id.value
        );

        assert_eq!(
            EntityId_t::new(EntityId_t::ENTITYID_P2P_BUILTIN_PARTICIPANT_STATELESS_READER).value,
            EntityId_t::participant_stateless_message_reader_entity_id.value
        );

        assert_eq!(
            EntityId_t::new(EntityId_t::ENTITYID_TL_SVC_REQ_WRITER).value,
            EntityId_t::c_EntityId_TypeLookup_request_writer.value
        );

        assert_eq!(
            EntityId_t::new(EntityId_t::ENTITYID_TL_SVC_REQ_READER).value,
            EntityId_t::c_EntityId_TypeLookup_request_reader.value
        );

        assert_eq!(
            EntityId_t::new(EntityId_t::ENTITYID_TL_SVC_REPLY_WRITER).value,
            EntityId_t::c_EntityId_TypeLookup_reply_writer.value
        );

        assert_eq!(
            EntityId_t::new(EntityId_t::ENTITYID_TL_SVC_REPLY_READER).value,
            EntityId_t::c_EntityId_TypeLookup_reply_reader.value
        );

        assert_eq!(
            EntityId_t::new(EntityId_t::ENTITYID_SEDP_BUILTIN_PUBLICATIONS_SECURE_WRITER).value,
            EntityId_t::sedp_builtin_publications_secure_writer.value
        );

        assert_eq!(
            EntityId_t::new(EntityId_t::ENTITYID_SEDP_BUILTIN_PUBLICATIONS_SECURE_READER).value,
            EntityId_t::sedp_builtin_publications_secure_reader.value
        );

        assert_eq!(
            EntityId_t::new(EntityId_t::ENTITYID_SEDP_BUILTIN_SUBSCRIPTIONS_SECURE_WRITER).value,
            EntityId_t::sedp_builtin_subscriptions_secure_writer.value
        );

        assert_eq!(
            EntityId_t::new(EntityId_t::ENTITYID_SEDP_BUILTIN_SUBSCRIPTIONS_SECURE_READER).value,
            EntityId_t::sedp_builtin_subscriptions_secure_reader.value
        );

        assert_eq!(
            EntityId_t::new(EntityId_t::ENTITYID_P2P_BUILTIN_PARTICIPANT_MESSAGE_SECURE_WRITER)
                .value,
            EntityId_t::c_EntityId_WriterLivelinessSecure.value
        );

        assert_eq!(
            EntityId_t::new(EntityId_t::ENTITYID_P2P_BUILTIN_PARTICIPANT_MESSAGE_SECURE_READER)
                .value,
            EntityId_t::c_EntityId_ReaderLivelinessSecure.value
        );

        assert_eq!(
            EntityId_t::new(
                EntityId_t::ENTITYID_P2P_BUILTIN_PARTICIPANT_VOLATILE_MESSAGE_SECURE_WRITER
            )
            .value,
            EntityId_t::participant_volatile_message_secure_writer_entity_id.value
        );

        assert_eq!(
            EntityId_t::new(
                EntityId_t::ENTITYID_P2P_BUILTIN_PARTICIPANT_VOLATILE_MESSAGE_SECURE_READER
            )
            .value,
            EntityId_t::participant_volatile_message_secure_reader_entity_id.value
        );

        assert_eq!(
            EntityId_t::new(EntityId_t::ENTITYID_SPDP_RELIABLE_BUILTIN_PARTICIPANT_SECURE_WRITER)
                .value,
            [0xffu8, 0x01u8, 0x01u8, 0xc2u8]
        );

        assert_eq!(
            EntityId_t::new(EntityId_t::ENTITYID_SPDP_RELIABLE_BUILTIN_PARTICIPANT_SECURE_READER)
                .value,
            [0xffu8, 0x01u8, 0x01u8, 0xc7u8]
        );

        assert_eq!(
            EntityId_t::new(EntityId_t::ENTITYID_DS_SERVER_VIRTUAL_WRITER).value,
            EntityId_t::ds_server_virtual_writer.value
        );

        assert_eq!(
            EntityId_t::new(EntityId_t::ENTITYID_DS_SERVER_VIRTUAL_READER).value,
            EntityId_t::ds_server_virtual_reader.value
        );
    }

    #[test]
    fn operator_test() {
        let id1 = EntityId_t::c_EntityId_SEDPPubWriter;
        assert_eq!(id1.value, EntityId_t::c_EntityId_SEDPPubWriter.value);

        let id2 = EntityId_t::c_EntityId_SEDPPubWriter.clone();
        assert_eq!(id1.value, id2.value);
        assert!(id1 == id2);

        let id3 = EntityId_t::unknown();
        assert!(id1 != id3);

        let id4 = EntityId_t::ENTITYID_RTPSParticipant;
        assert!(id1 != id4);
        assert!(id4 != id1);
        assert!(id1 == EntityId_t::ENTITYID_SEDP_BUILTIN_PUBLICATIONS_WRITER);
        assert!(EntityId_t::ENTITYID_SEDP_BUILTIN_PUBLICATIONS_WRITER == id1);
    }
}
