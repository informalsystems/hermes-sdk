use alloc::string::String;
use cgp::prelude::*;
use hermes_chain_type_components::traits::types::address::HasAddressType;
use hermes_chain_type_components::traits::types::amount::HasAmountType;
use hermes_chain_type_components::traits::types::denom::HasDenomType;
use hermes_chain_type_components::traits::types::ibc::channel_id::HasChannelIdType;
use hermes_ibc_components::traits::fields::message::app_id::HasIbcMessageAppIds;
use hermes_ibc_components::traits::fields::packet::header::channel_id::HasPacketChannelIds;
use hermes_ibc_components::traits::fields::packet::header::nonce::HasPacketNonce;
use hermes_ibc_components::traits::fields::packet::header::timeout::HasPacketTimeout;
use hermes_ibc_components::traits::fields::packet::packet::payloads::HasPacketPayloads;
use hermes_ibc_components::traits::fields::payload::app_id::HasPayloadAppIds;
use hermes_ibc_components::traits::types::app_id::HasAppIdType;
use hermes_ibc_components::traits::types::message_header::HasIbcMessageHeaderType;
use hermes_ibc_components::traits::types::packet::header::HasPacketHeaderType;
use hermes_ibc_components::traits::types::packet::nonce::HasPacketNonceType;
use hermes_ibc_components::traits::types::packet::packet::HasPacketType;
use hermes_ibc_components::traits::types::packet::timeout::HasPacketTimeoutType;
use hermes_ibc_components::traits::types::payload::data::HasPayloadDataType;
use hermes_ibc_components::traits::types::payload::header::HasPayloadHeaderType;
use hermes_ibc_components::types::any_app::AnyApp;
use hermes_ibc_components::types::message_header::IbcMessageHeader;
use hermes_ibc_components::types::packet::IbcPacket;
use hermes_ibc_components::types::packet_header::IbcPacketHeader;
use hermes_ibc_components::types::payload_header::IbcPayloadHeader;
use hermes_ibc_token_transfer_components::types::packet_data::mint::IbcTransferMintPacketData;
use hermes_ibc_token_transfer_components::types::packet_data::transfer::IbcTransferPacketData;
use hermes_ibc_token_transfer_components::types::packet_data::unescrow::IbcTransferUnescrowPacketData;
use hermes_ibc_token_transfer_components::types::tags::{
    IbcTransferApp, IbcTransferMintApp, IbcTransferUnescrowApp,
};

use crate::components::chain::MockChainComponents;
use crate::types::address::MockAddress;
use crate::types::amount::MockAmount;
use crate::types::app_id::MockAppId;
use crate::types::channel_id::MockChannelId;
use crate::types::denom::MockDenom;
use crate::types::height::MockHeight;
use crate::types::nonce::MockNonce;
use crate::types::packet_data::MockAnyPacketData;
use crate::types::tagged::Tagged;
use crate::types::tags::{ChainA, ChainB};

pub struct MockChain;

impl<Chain, Counterparty> HasComponents for Tagged<Chain, Counterparty, MockChain> {
    type Components = MockChainComponents<Chain, Counterparty>;
}

pub type MockChainA = Tagged<ChainA, ChainB, MockChain>;
pub type MockChainB = Tagged<ChainB, ChainA, MockChain>;

pub trait CanUseMockChain: HasErrorType<Error = String>
    + HasAddressType<Address = Tagged<ChainA, ChainB, MockAddress>>
    + HasDenomType<Denom = Tagged<ChainA, ChainB, MockDenom>>
    + HasAmountType<Amount = Tagged<ChainA, ChainB, MockAmount>>
    + HasAppIdType<MockChainB, AppId = Tagged<ChainA, ChainB, MockAppId>>
    + HasChannelIdType<MockChainB, ChannelId = Tagged<ChainA, ChainB, MockChannelId>>
    + HasPacketTimeoutType<MockChainB, PacketTimeout = Tagged<ChainA, ChainB, MockHeight>>
    + HasPacketNonceType<MockChainB, PacketNonce = Tagged<ChainA, ChainB, MockNonce>>
    + HasPacketType<MockChainB, Packet = IbcPacket<MockChainA, MockChainB, AnyApp>>
    + HasPacketHeaderType<MockChain, PacketHeader = IbcPacketHeader<MockChainA, MockChainB>>
    + HasPayloadHeaderType<MockChain, PayloadHeader = IbcPayloadHeader<MockChainA, MockChainB>>
    + HasIbcMessageHeaderType<MockChain, IbcMessageHeader = IbcMessageHeader<MockChainA, MockChainB>>
    + HasPacketPayloads<MockChain, AnyApp>
    + HasPayloadDataType<MockChain, AnyApp, PayloadData = MockAnyPacketData<ChainA, ChainB>>
    + HasPayloadDataType<
        MockChain,
        IbcTransferApp,
        PayloadData = IbcTransferPacketData<MockChainA, MockChainB>,
    > + HasPayloadDataType<
        MockChain,
        IbcTransferMintApp,
        PayloadData = IbcTransferMintPacketData<MockChainA, MockChainB>,
    > + HasPayloadDataType<
        MockChain,
        IbcTransferUnescrowApp,
        PayloadData = IbcTransferUnescrowPacketData<MockChainB>,
    > + HasPacketChannelIds<MockChainB>
    + HasPacketNonce<MockChainB>
    + HasPacketTimeout<MockChainB>
    + HasPayloadAppIds<MockChainB>
    + HasIbcMessageAppIds<MockChainB>
{
}

impl CanUseMockChain for MockChainA {}
