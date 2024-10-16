use core::marker::PhantomData;
use core::ops::Deref;

use alloc::collections::btree_map::BTreeMap;
use alloc::string::String;
use alloc::sync::Arc;
use cgp::prelude::*;
use hermes_chain_type_components::traits::types::address::HasAddressType;
use hermes_chain_type_components::traits::types::amount::HasAmountType;
use hermes_chain_type_components::traits::types::denom::HasDenomType;
use hermes_chain_type_components::traits::types::height::HasHeightType;
use hermes_chain_type_components::traits::types::ibc::channel_id::HasChannelIdType;
use hermes_ibc_components::traits::fields::message::app_id::HasIbcMessageAppIds;
use hermes_ibc_components::traits::fields::packet::header::channel_id::HasPacketChannelIds;
use hermes_ibc_components::traits::fields::packet::header::timeout::HasPacketTimeout;
use hermes_ibc_components::traits::fields::packet::packet::nonce::HasPacketNonce;
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
use hermes_ibc_components::traits::types::payload::payload::HasPayloadType;
use hermes_ibc_components::types::any_app::AnyApp;
use hermes_ibc_components::types::message_header::IbcMessageHeader;
use hermes_ibc_components::types::packet::IbcPacket;
use hermes_ibc_components::types::packet_header::IbcPacketHeader;
use hermes_ibc_components::types::payload::IbcPayload;
use hermes_ibc_components::types::payload_header::IbcPayloadHeader;
use hermes_ibc_token_transfer_components::types::packet_data::mint::IbcTransferMintPayloadData;
use hermes_ibc_token_transfer_components::types::packet_data::transfer::IbcTransferPayloadData;
use hermes_ibc_token_transfer_components::types::packet_data::unescrow::IbcTransferUnescrowPayloadData;
use hermes_ibc_token_transfer_components::types::tags::{
    IbcTransferApp, IbcTransferMintApp, IbcTransferUnescrowApp,
};

use crate::components::chain::MockChainComponents;
use crate::types::address::MockAddress;
use crate::types::amount::MockAmount;
use crate::types::app_id::MockAppId;
use crate::types::channel_id::MockChannelId;
use crate::types::client_id::MockClientId;
use crate::types::denom::MockDenom;
use crate::types::height::MockHeight;
use crate::types::nonce::MockNonce;
use crate::types::packet_data::MockAnyPayloadData;
use crate::types::tagged::Tagged;
use crate::types::tags::{ChainA, ChainB};

pub struct MockChain<Chain: Async, Counterparty: Async> {
    pub state: Arc<dyn HasMockChainState<Chain, Counterparty>>,
    pub phantom: PhantomData<(Chain, Counterparty)>,
}

#[derive(HasField)]
pub struct MockChainState<Chain: Async, Counterparty: Async> {
    pub current_height: MockHeight,
    pub channel_clients: BTreeMap<
        Tagged<Chain, Counterparty, MockChannelId>,
        Tagged<Chain, Counterparty, MockClientId>,
    >,
    pub consensus_states: BTreeMap<
        Tagged<Chain, Counterparty, MockClientId>,
        BTreeMap<Tagged<Counterparty, Chain, MockHeight>, MockChainState<Counterparty, Chain>>,
    >,
    pub received_packets: BTreeMap<
        (
            Tagged<Chain, Counterparty, MockChannelId>,
            Tagged<Counterparty, Chain, MockChannelId>,
            Tagged<Chain, Counterparty, MockAppId>,
            Tagged<Counterparty, Chain, MockAppId>,
        ),
        BTreeMap<
            Tagged<Counterparty, Chain, MockNonce>,
            IbcPacket<MockChain<Counterparty, Chain>, MockChain<Chain, Counterparty>>,
        >,
    >,
    pub sent_packets: BTreeMap<
        (
            Tagged<Chain, Counterparty, MockChannelId>,
            Tagged<Counterparty, Chain, MockChannelId>,
            Tagged<Chain, Counterparty, MockAppId>,
            Tagged<Counterparty, Chain, MockAppId>,
        ),
        BTreeMap<
            Tagged<Chain, Counterparty, MockNonce>,
            IbcPacket<MockChain<Chain, Counterparty>, MockChain<Counterparty, Chain>>,
        >,
    >,
}

/**
   This is a type wrapper to allow [`MockChain`] to contain fields that refer
   to `Self`, such as [`IbcPacket`], via [`MockChainState`].

   By moving the fields to a separate [`MockChainState`] struct and wrap it inside
   a `dyn HasMockChainState`, we prevent the Rust compiler from overflowing the
   trait resolution when resolving the trait bound of the mock chain fields.
*/
pub trait HasMockChainState<Chain: Async, Counterparty: Async>: Send + Sync + 'static {
    fn mock_chain_state(&self) -> &MockChainState<Chain, Counterparty>;
}

impl<Chain: Async, Counterparty: Async> HasMockChainState<Chain, Counterparty>
    for MockChainState<Chain, Counterparty>
{
    fn mock_chain_state(&self) -> &MockChainState<Chain, Counterparty> {
        self
    }
}

impl<Chain: Async, Counterparty: Async> Deref for MockChain<Chain, Counterparty> {
    type Target = MockChainState<Chain, Counterparty>;

    fn deref(&self) -> &Self::Target {
        self.state.mock_chain_state()
    }
}

impl<Chain: Async, Counterparty: Async> HasComponents for MockChain<Chain, Counterparty> {
    type Components = MockChainComponents;
}

impl<Chain: Async, Counterparty: Async> Clone for MockChainState<Chain, Counterparty> {
    fn clone(&self) -> Self {
        Self {
            current_height: self.current_height.clone(),
            channel_clients: self.channel_clients.clone(),
            consensus_states: self.consensus_states.clone(),
            received_packets: self.received_packets.clone(),
            sent_packets: self.sent_packets.clone(),
        }
    }
}

impl<Chain: Async, Counterparty: Async> Clone for MockChain<Chain, Counterparty> {
    fn clone(&self) -> Self {
        Self {
            state: Arc::new(self.state.mock_chain_state().clone()),
            phantom: PhantomData,
        }
    }
}

impl<Chain: Async, Counterparty: Async> Default for MockChain<Chain, Counterparty> {
    fn default() -> Self {
        Self {
            state: Arc::new(MockChainState {
                current_height: MockHeight(0),
                channel_clients: BTreeMap::default(),
                consensus_states: BTreeMap::default(),
                received_packets: BTreeMap::default(),
                sent_packets: BTreeMap::default(),
            }),
            phantom: PhantomData,
        }
    }
}

impl<Chain: Async, Counterparty: Async> MockChain<Chain, Counterparty> {
    pub fn fork(&self) -> Self {
        Self {
            state: self.state.clone(),
            phantom: PhantomData,
        }
    }
}

pub type MockChainA = MockChain<ChainA, ChainB>;
pub type MockChainB = MockChain<ChainB, ChainA>;

pub trait CanUseMockChain: HasErrorType<Error = String>
    + HasHeightType<Height = Tagged<ChainA, ChainB, MockHeight>>
    + HasAddressType<Address = Tagged<ChainA, ChainB, MockAddress>>
    + HasDenomType<Denom = MockDenom<ChainA, ChainB>>
    + HasAmountType<Amount = MockAmount<ChainA, ChainB>>
    + HasAppIdType<MockChainB, AppId = Tagged<ChainA, ChainB, MockAppId>>
    + HasChannelIdType<MockChainB, ChannelId = Tagged<ChainA, ChainB, MockChannelId>>
    + HasPacketTimeoutType<MockChainB, PacketTimeout = Tagged<ChainA, ChainB, MockHeight>>
    + HasPacketNonceType<MockChainB, PacketNonce = Tagged<ChainA, ChainB, MockNonce>>
    + HasPacketType<MockChainB, Packet = IbcPacket<MockChainA, MockChainB>>
    + HasPacketHeaderType<MockChainB, PacketHeader = IbcPacketHeader<MockChainA, MockChainB>>
    + HasPayloadHeaderType<MockChainB, PayloadHeader = IbcPayloadHeader<MockChainA, MockChainB>>
    + HasIbcMessageHeaderType<MockChainB, IbcMessageHeader = IbcMessageHeader<MockChainA, MockChainB>>
    + HasPacketPayloads<MockChainB>
    + HasPayloadType<MockChainB, Payload = IbcPayload<MockChainA, MockChainB, AnyApp>>
    + HasPayloadDataType<MockChainB, AnyApp, PayloadData = MockAnyPayloadData<ChainA, ChainB>>
    + HasPayloadDataType<
        MockChainB,
        IbcTransferApp,
        PayloadData = IbcTransferPayloadData<MockChainA, MockChainB>,
    > + HasPayloadDataType<
        MockChainB,
        IbcTransferMintApp,
        PayloadData = IbcTransferMintPayloadData<MockChainA, MockChainB>,
    > + HasPayloadDataType<
        MockChainB,
        IbcTransferUnescrowApp,
        PayloadData = IbcTransferUnescrowPayloadData<MockChainA, MockChainB>,
    > + HasPacketChannelIds<MockChainB>
    + HasPacketNonce<MockChainB>
    + HasPacketTimeout<MockChainB>
    + HasPayloadAppIds<MockChainB>
    + HasIbcMessageAppIds<MockChainB>
{
}

impl CanUseMockChain for MockChainA {}
