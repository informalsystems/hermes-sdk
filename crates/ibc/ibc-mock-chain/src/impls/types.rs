use alloc::string::String;
use cgp::core::error::ErrorTypeComponent;
use cgp::prelude::*;
use hermes_chain_type_components::traits::types::ibc::channel_id::ChannelIdTypeComponent;
use hermes_ibc_components::traits::types::app_id::AppIdTypeComponent;
use hermes_ibc_components::traits::types::packet::header::PacketHeaderTypeComponent;
use hermes_ibc_components::traits::types::packet::nonce::PacketNonceTypeComponent;
use hermes_ibc_components::types::packet_header::PacketHeader;

use crate::contexts::chain::MockChain;
use crate::types::app_id::MockAppId;
use crate::types::channel_id::MockChannelId;

define_components! {
    MockChainTypes {
        ErrorTypeComponent: String,
        AppIdTypeComponent: MockAppId,
        ChannelIdTypeComponent: MockChannelId,
        PacketNonceTypeComponent: u8,
        PacketHeaderTypeComponent: PacketHeader<MockChain, MockChain>,
    }
}
