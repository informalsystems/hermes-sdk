use alloc::string::String;
use cgp::core::error::ErrorTypeComponent;
use cgp::prelude::*;
use hermes_chain_type_components::traits::types::ibc::channel_id::ChannelIdTypeComponent;
use hermes_ibc_components::traits::types::app_id::AppIdTypeComponent;
use hermes_ibc_components::traits::types::message_header::IbcMessageHeaderTypeComponent;
use hermes_ibc_components::traits::types::packet::header::PacketHeaderTypeComponent;
use hermes_ibc_components::traits::types::packet::nonce::PacketNonceTypeComponent;
use hermes_ibc_components::traits::types::packet::packet::PacketTypeComponent;
use hermes_ibc_components::traits::types::packet::timeout::PacketTimeoutTypeComponent;
use hermes_ibc_components::traits::types::payload::data::PayloadDataTypeComponent;
use hermes_ibc_components::traits::types::payload::header::PayloadHeaderTypeComponent;
use hermes_ibc_components::types::any_app::AnyApp;
use hermes_ibc_components::types::message_header::IbcMessageHeader;
use hermes_ibc_components::types::packet::IbcPacket;
use hermes_ibc_components::types::packet_header::IbcPacketHeader;
use hermes_ibc_components::types::payload_header::IbcPayloadHeader;

use crate::contexts::chain::MockChain;
use crate::types::app_id::MockAppId;
use crate::types::channel_id::MockChannelId;
use crate::types::packet_data::MockAnyPacketData;

define_components! {
    MockChainTypes {
        ErrorTypeComponent: String,
        AppIdTypeComponent: MockAppId,
        ChannelIdTypeComponent: MockChannelId,
        PacketNonceTypeComponent: u8,
        PacketTimeoutTypeComponent: u8,
        PacketTypeComponent: IbcPacket<MockChain, MockChain, AnyApp>,
        PacketHeaderTypeComponent: IbcPacketHeader<MockChain, MockChain>,
        PayloadHeaderTypeComponent: IbcPayloadHeader<MockChain, MockChain>,
        PayloadDataTypeComponent: MockAnyPacketData,
        IbcMessageHeaderTypeComponent: IbcMessageHeader<MockChain, MockChain>,
    }
}
