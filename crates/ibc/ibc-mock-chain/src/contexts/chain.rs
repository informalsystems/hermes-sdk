use alloc::string::String;
use cgp::prelude::*;
use hermes_chain_type_components::traits::types::ibc::channel_id::HasChannelIdType;
use hermes_ibc_components::traits::fields::packet::header::channel_id::HasPacketChannelIds;
use hermes_ibc_components::traits::fields::packet::header::nonce::HasPacketNonce;
use hermes_ibc_components::traits::types::app_id::HasAppIdType;
use hermes_ibc_components::traits::types::packet::header::HasPacketHeaderType;
use hermes_ibc_components::types::packet_header::PacketHeader;

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
    + HasPacketHeaderType<MockChain, PacketHeader = PacketHeader<MockChain, MockChain>>
    + HasPacketChannelIds<MockChain>
    + HasPacketNonce<MockChain>
{
}

impl CanUseMockChain for MockChain {}
