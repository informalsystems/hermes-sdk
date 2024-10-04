use cgp::prelude::*;
use hermes_ibc_components::types::any_app::AnyApp;

use crate::types::packet_data::MockAnyPacketData;

define_components! {
    PacketDataTypes {
        AnyApp: MockAnyPacketData,
    }
}
