use alloc::string::String;
use cgp::prelude::*;
use hermes_chain_type_components::traits::types::ibc::channel_id::HasChannelIdType;
use hermes_ibc_components::traits::fields::packet::header::channel_id::HasPacketChannelIds;
use hermes_ibc_components::traits::fields::packet::header::nonce::HasPacketNonce;
use hermes_ibc_components::traits::fields::packet::header::timeout::HasPacketTimeout;
use hermes_ibc_components::traits::fields::packet::packet::payloads::HasPacketPayloads;
use hermes_ibc_components::traits::fields::payload::app_id::HasPayloadAppIds;
use hermes_ibc_components::traits::types::app_id::HasAppIdType;
use hermes_ibc_components::traits::types::packet::header::HasPacketHeaderType;
use hermes_ibc_components::traits::types::packet::nonce::HasPacketNonceType;
use hermes_ibc_components::traits::types::packet::packet::HasPacketType;
use hermes_ibc_components::traits::types::packet::timeout::HasPacketTimeoutType;
use hermes_ibc_components::traits::types::payload::data::HasPayloadDataType;
use hermes_ibc_components::traits::types::payload::header::HasPayloadHeaderType;
use hermes_ibc_components::types::any_app::AnyApp;
use hermes_ibc_components::types::packet::IbcPacket;
use hermes_ibc_components::types::packet_header::IbcPacketHeader;
use hermes_ibc_components::types::payload_header::IbcPayloadHeader;

use crate::components::chain::MockChainComponents;
use crate::types::app_id::MockAppId;
use crate::types::channel_id::MockChannelId;

pub struct MockChain;

impl HasComponents for MockChain {
    type Components = MockChainComponents;
}

pub trait CanUseMockChain:
    HasErrorType<Error = String>
    + HasAppIdType<MockChain, AppId = MockAppId>
    + HasChannelIdType<MockChain, ChannelId = MockChannelId>
    + HasPacketTimeoutType<MockChain, PacketTimeout = u8>
    + HasPacketNonceType<MockChain, PacketNonce = u8>
    + HasPacketType<MockChain, Packet = IbcPacket<MockChain, MockChain, AnyApp>>
    + HasPacketHeaderType<MockChain, PacketHeader = IbcPacketHeader<MockChain, MockChain>>
    + HasPayloadHeaderType<MockChain, PayloadHeader = IbcPayloadHeader<MockChain, MockChain>>
    + HasPacketPayloads<MockChain, AnyApp>
    + HasPayloadDataType<MockChain, AnyApp>
    + HasPacketChannelIds<MockChain>
    + HasPacketNonce<MockChain>
    + HasPacketTimeout<MockChain>
    + HasPayloadAppIds<MockChain>
{
}

impl CanUseMockChain for MockChain {}
